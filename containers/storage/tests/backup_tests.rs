use std::fs;
use tempfile::tempdir;
use verseguy_storage::BackupService;

#[test]
fn test_create_and_restore_backup() -> anyhow::Result<()> {
    let td = tempdir()?;
    let db_dir = td.path().join("db");
    fs::create_dir_all(&db_dir)?;
    // create some dummy file
    fs::write(db_dir.join("hello.txt"), b"world")?;

    let backups = td.path().join("backups");
    let svc = BackupService::new(&db_dir, &backups);

    let bfile = svc.create_backup()?;
    assert!(bfile.exists());

    // Remove db_dir and restore
    fs::remove_dir_all(&db_dir)?;
    svc.restore_backup(&bfile, &db_dir)?;

    let content = fs::read(db_dir.join("hello.txt"))?;
    assert_eq!(content, b"world");
    Ok(())
}
