use anyhow::Result;
use chrono::{DateTime, Utc};
use quick_xml::Reader;
use quick_xml::events::Event;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;
use std::path::Path;
use std::process::Command;
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

/// Import all JSON files from a public GitHub repository (owner/repo[@branch]).
/// It will iterate the repository tree and import files ending with `.json`.
pub fn import_from_github(repo: &str, storage: &RocksDBStorage) -> Result<usize> {
    // parse owner/repo[@branch]
    let parts: Vec<&str> = repo.split('@').collect();
    let repo_part = parts[0];
    let branch = if parts.len() > 1 { parts[1] } else { "main" };

    let mut sp = repo_part.split('/');
    let owner = sp
        .next()
        .ok_or_else(|| anyhow::anyhow!("invalid repo format, expected owner/repo"))?;
    let repo_name = sp
        .next()
        .ok_or_else(|| anyhow::anyhow!("invalid repo format, expected owner/repo"))?;

    let client = Client::builder()
        .user_agent("verseguy-scunpacked-importer/0.1")
        .build()?;

    let tree_url = format!(
        "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
        owner, repo_name, branch
    );
    info!("listing repo tree {}", tree_url);
    let resp = client.get(&tree_url).send()?;
    if !resp.status().is_success() {
        warn!("failed to list repo tree: {}", resp.status());
        return Ok(0);
    }
    let v: Value = resp.json()?;
    let tree = v
        .get("tree")
        .and_then(|t| t.as_array())
        .ok_or_else(|| anyhow::anyhow!("no tree in response"))?;

    let mut total = 0usize;
    for entry in tree {
        if let Some(path) = entry.get("path").and_then(|p| p.as_str())
            && path.ends_with(".json")
        {
            let raw_url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo_name, branch, path
            );
            info!("fetching {}", raw_url);
            let r = client.get(&raw_url).send()?;
            if !r.status().is_success() {
                warn!("failed to fetch {}: {}", raw_url, r.status());
                continue;
            }
            let body = r.text()?;
            match import_from_reader(body.as_bytes(), storage) {
                Ok(n) => total += n,
                Err(e) => warn!("skipping {} due to error: {}", path, e),
            }
        }
    }

    info!("imported {} items from {}/{}", total, owner, repo_name);
    Ok(total)
}

/// Import ships from an already-extracted directory (XML files present). This converts simple ship XMLs into ScShip and imports them.
pub fn import_from_extracted_dir<P: AsRef<Path>>(
    dir: P,
    storage: &RocksDBStorage,
) -> Result<usize> {
    let dirp = dir.as_ref();
    if !dirp.exists() {
        return Ok(0);
    }
    let mut total = 0usize;
    for entry in std::fs::read_dir(dirp)? {
        let p = entry?.path();
        if p.is_file()
            && p.extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("xml"))
                .unwrap_or(false)
        {
            // parse XML for simple ship fields
            let mut reader = Reader::from_file(&p)?;
            reader.trim_text(true);
            let mut buf = Vec::new();
            let mut current_tag: Option<String> = None;
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut role: Option<String> = None;
            loop {
                match reader.read_event_into(&mut buf) {
                    Ok(Event::Start(e)) => {
                        current_tag = Some(
                            String::from_utf8_lossy(e.name().as_ref())
                                .to_string()
                                .to_lowercase(),
                        );
                    }
                    Ok(Event::Text(e)) => {
                        if let Some(tag) = current_tag.as_ref() {
                            let txt = e.unescape().unwrap_or_default().into_owned();
                            match tag.as_str() {
                                "classname" | "class" | "id" | "classid" => {
                                    if id.is_none() {
                                        id = Some(txt.clone());
                                    }
                                }
                                "displayname" | "name" | "label" | "display_name" => {
                                    if name.is_none() {
                                        name = Some(txt.clone());
                                    }
                                }
                                "role" => {
                                    if role.is_none() {
                                        role = Some(txt.clone());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Ok(Event::Eof) => break,
                    Err(e) => {
                        warn!("xml parse error {}: {}", p.display(), e);
                        break;
                    }
                    _ => {}
                }
                buf.clear();
            }
            let idv = id.or_else(|| {
                p.file_stem()
                    .and_then(|s| s.to_str().map(|s| s.to_string()))
            });
            let namev = name.or_else(|| idv.clone());
            if let Some(idv) = idv {
                let ship = ScShip {
                    id: idv.clone(),
                    name: namev.unwrap_or_else(|| idv.clone()),
                    role: role.clone(),
                    stats: None,
                    variants: None,
                };
                let k = ship_key(&ship.id);
                let existing: Option<ScShip> = storage.get(k.as_bytes())?;
                if existing.as_ref() != Some(&ship) {
                    storage.put(k.as_bytes(), &ship)?;
                    total += 1;
                }
            }
        }
    }
    if total > 0 {
        let now: DateTime<Utc> = Utc::now();
        storage.put(meta_key(), &now.to_rfc3339())?;
    }
    Ok(total)
}

/// Import directly from a .p4k archive using `unp4k` from PATH. If `unp4k` isn't available, returns an error explaining the requirement.
pub fn import_from_p4k<P: AsRef<Path>>(p4k: P, storage: &RocksDBStorage) -> Result<usize> {
    let p4k = p4k.as_ref();
    if !p4k.exists() {
        return Err(anyhow::anyhow!("p4k path does not exist"));
    }

    // check unp4k availability
    let which = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };
    let which_out = Command::new(which).arg("unp4k").output();
    if which_out.is_err() || !which_out.unwrap().status.success() {
        return Err(anyhow::anyhow!(
            "`unp4k` not found in PATH. Please install https://github.com/dolkensp/unp4k and ensure it's on PATH."
        ));
    }

    let tmpdir = tempfile::tempdir()?;
    let outdir = tmpdir.path();
    // extract only xml and ini (per scunpacked docs)
    // Use current_dir to place extracted files into outdir
    let status = Command::new("unp4k")
        .current_dir(outdir)
        .arg(p4k)
        .arg("*.xml")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("unp4k failed to extract files"));
    }

    // now import from extracted directory
    let n = import_from_extracted_dir(outdir, storage)?;
    Ok(n)
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
