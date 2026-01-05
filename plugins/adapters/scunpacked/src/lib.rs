use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::Read;
use tracing::{info, warn};
use verseguy_storage::RocksDBStorage;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ScShip {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    // extended fields
    pub stats: Option<ScStats>,
    pub variants: Option<Vec<ScVariant>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ScStats {
    pub mass: Option<f64>,
    pub cargo: Option<u64>,
}

// Keep the top-level types Eq-capable by not deriving Eq for ScShip; instead compare via PartialEq

// Note: f64 does not implement Eq, so we keep PartialEq only for stats

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ScVariant {
    pub id: String,
    pub name: String,
}

fn ship_key(id: &str) -> String {
    format!("scunpacked:ship:{}", id)
}

fn meta_key() -> &'static [u8] {
    b"scunpacked:meta:last_updated"
}

/// Import ships from any reader (JSON array)
pub fn import_from_reader<R: Read>(mut rdr: R, storage: &RocksDBStorage) -> Result<usize> {
    let mut data = String::new();
    rdr.read_to_string(&mut data)?;
    let ships: Vec<ScShip> = serde_json::from_str(&data)?;
    let mut count = 0usize;

    for s in ships {
        let k = ship_key(&s.id);
        // upsert: avoid writing if identical
        let existing: Option<ScShip> = storage.get(k.as_bytes())?;
        if existing.as_ref() == Some(&s) {
            info!("skipping unchanged ship {}", s.id);
            continue;
        }
        storage.put(k.as_bytes(), &s)?;
        count += 1;
    }
    // update meta timestamp
    let now: DateTime<Utc> = Utc::now();
    storage.put(meta_key(), &now.to_rfc3339())?;
    info!(
        "imported {} ships, updated meta to {}",
        count,
        now.to_rfc3339()
    );
    Ok(count)
}

/// Import from a local file path
pub fn import_from_file<P: AsRef<str>>(path: P, storage: &RocksDBStorage) -> Result<usize> {
    let f = std::fs::File::open(path.as_ref())?;
    import_from_reader(f, storage)
}

/// Import from a HTTP(S) URL. If `if_newer_than` is provided (rfc3339), only imports when remote-modified is newer.
pub fn import_from_url<P: AsRef<str>>(
    url: P,
    storage: &RocksDBStorage,
    if_newer_than: Option<&str>,
) -> Result<usize> {
    let url_s = url.as_ref();
    info!("fetching URL {}", url_s);
    let client = Client::builder().build()?;
    let resp = client.get(url_s).send()?;
    if !resp.status().is_success() {
        warn!("failed to download {}: status={} ", url_s, resp.status());
        return Ok(0);
    }
    // If 'if_newer_than' is provided, check Last-Modified header if present
    if let Some(ref_if) = if_newer_than
        && let Some(h) = resp.headers().get(reqwest::header::LAST_MODIFIED)
        && let Ok(s) = h.to_str()
        && let Ok(lm) = DateTime::parse_from_rfc2822(s)
        && let Ok(if_dt) = DateTime::parse_from_rfc3339(ref_if)
    {
        let lm_utc: DateTime<Utc> = lm.with_timezone(&Utc);
        let if_utc = if_dt.with_timezone(&Utc);
        if lm_utc <= if_utc {
            info!("remote not newer (last-modified={}), skipping", lm_utc);
            return Ok(0);
        }
    }

    let body = resp.text()?;
    import_from_reader(body.as_bytes(), storage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn import_sample_file() {
        let dir = tempdir().unwrap();
        let dbpath = dir.path().join("db");
        let storage = RocksDBStorage::open(dbpath).unwrap();

        // prepare sample JSON
        let sample = r#"[
            {"id":"ship-a","name":"Ship A","role":"combat"},
            {"id":"ship-b","name":"Ship B","role":"transport"}
        ]"#;
        let file = dir.path().join("sample.json");
        std::fs::write(&file, sample).unwrap();

        let n = import_from_file(file.to_str().unwrap(), &storage).unwrap();
        assert_eq!(n, 2);

        let s: Option<ScShip> = storage.get(b"scunpacked:ship:ship-a").unwrap();
        assert!(s.is_some());
        assert_eq!(s.unwrap().name, "Ship A");
    }
}
