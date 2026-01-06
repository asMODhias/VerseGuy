use filetime::FileTime;
use std::fs;
use std::time::{Duration, SystemTime};
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

    let bfile = svc.create_backup(None)?;
    assert!(bfile.exists());

    // Remove db_dir and restore
    fs::remove_dir_all(&db_dir)?;
    svc.restore_backup(&bfile, &db_dir, None)?;

    let content = fs::read(db_dir.join("hello.txt"))?;
    assert_eq!(content, b"world");
    Ok(())
}

#[test]
fn test_create_and_restore_encrypted_backup() -> anyhow::Result<()> {
    let td = tempdir()?;
    let db_dir = td.path().join("db_enc");
    fs::create_dir_all(&db_dir)?;
    fs::write(db_dir.join("secret.txt"), b"squirrel")?;

    let backups = td.path().join("backups_enc");
    let svc = BackupService::new(&db_dir, &backups);

    let key = [42u8; 32];
    let bfile = svc.create_backup(Some(&key))?;
    assert!(bfile.exists());
    let meta = fs::metadata(&bfile)?;
    println!("Encrypted backup path: {:?}, size: {}", bfile, meta.len());

    // Remove db_dir and restore
    fs::remove_dir_all(&db_dir)?;
    svc.restore_backup(&bfile, &db_dir, Some(&key))?;

    let content = fs::read(db_dir.join("secret.txt"))?;
    assert_eq!(content, b"squirrel");
    Ok(())
}

#[test]
fn test_cleanup_old_backups_keeps_n() -> anyhow::Result<()> {
    let td = tempdir()?;
    let db_dir = td.path().join("db_cleanup");
    fs::create_dir_all(&db_dir)?;
    fs::write(db_dir.join("f.txt"), b"x")?;

    let backups = td.path().join("backups_cleanup");
    let svc = BackupService::new(&db_dir, &backups);

    // create a few backups and tweak mtimes
    let b1 = svc.create_backup(None)?;
    std::thread::sleep(Duration::from_millis(10));
    let b2 = svc.create_backup(None)?;
    std::thread::sleep(Duration::from_millis(10));
    let b3 = svc.create_backup(None)?;

    // set mtimes to different times so ordering is deterministic
    let now = SystemTime::now();
    filetime::set_file_mtime(
        &b1,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )?;
    filetime::set_file_mtime(
        &b2,
        FileTime::from_system_time(now - Duration::from_secs(20)),
    )?;
    filetime::set_file_mtime(
        &b3,
        FileTime::from_system_time(now - Duration::from_secs(10)),
    )?;

    let deleted = svc.cleanup_old_backups(2)?;
    assert_eq!(deleted, 1);
    Ok(())
}
