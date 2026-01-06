use plugins_adapter_scunpacked::ScShip;
use plugins_adapter_scunpacked::import_from_file;
use std::path::PathBuf;
use tempfile::tempdir;
use verseguy_storage::RocksDBStorage;

fn main() -> anyhow::Result<()> {
    // Determine the sample file shipped with the crate
    let manifest = env!("CARGO_MANIFEST_DIR");
    let sample = PathBuf::from(manifest)
        .join("tests")
        .join("fixtures")
        .join("sample_ships.json");

    println!("Using sample file: {}", sample.display());

    let dir = tempdir()?;
    let dbpath = dir.path().join("db");
    let storage = RocksDBStorage::open(&dbpath)?;

    let n = import_from_file(sample.to_str().ok_or_else(|| anyhow::anyhow!("sample path not utf8"))?, &storage)?;
    println!("Imported {} ships", n);

    // Print out stored ships
    let ships: Vec<ScShip> = storage.prefix_scan(b"scunpacked:ship:")?;
    for s in ships {
        println!("- {}: {} ({:?})", s.id, s.name, s.role);
    }

    Ok(())
}
