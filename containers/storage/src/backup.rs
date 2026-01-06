use anyhow::{Context, Result};
use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};

use std::path::{Path, PathBuf};
use tar::Builder;

/// Simple backup service that creates a tar.gz archive of the DB path
pub struct BackupService {
    db_path: PathBuf,
    backup_dir: PathBuf,
}

impl BackupService {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(db_path: P, backup_dir: Q) -> Self {
        Self {
            db_path: db_path.as_ref().to_path_buf(),
            backup_dir: backup_dir.as_ref().to_path_buf(),
        }
    }

    pub fn create_backup(&self) -> Result<PathBuf> {
        fs::create_dir_all(&self.backup_dir)
            .context("Failed to create backup directory")?;
        let ts = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_id = format!("backup_{}", ts);
        let filename = format!("{}.tar.gz", backup_id);
        let backup_path = self.backup_dir.join(&filename);

        let tar_gz = File::create(&backup_path).context("Failed to create backup file")?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(enc);
        tar.append_dir_all("data", &self.db_path)
            .context("Failed to append db dir to tar")?;
        tar.finish().context("Failed to finish tar builder")?;

        Ok(backup_path)
    }

    pub fn restore_backup<P: AsRef<Path>>(&self, backup_path: P, restore_to: P) -> Result<()> {
        let backup_path = backup_path.as_ref();
        let restore_to = restore_to.as_ref();
        if !backup_path.exists() {
            anyhow::bail!("Backup not found: {:?}", backup_path);
        }

        // create temp dir
        let tmp = tempfile::tempdir().context("Failed to create tempdir")?;
        let tar_gz = File::open(backup_path).context("Failed to open backup file")?;
        let dec = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(dec);
        archive
            .unpack(&tmp.path())
            .context("Failed to unpack archive")?;

        // replace restore_to with extracted data/data
        let extracted = tmp.path().join("data");
        if restore_to.exists() {
            fs::remove_dir_all(restore_to).context("Failed to remove existing db path")?;
        }
        fs::rename(&extracted, restore_to).context("Failed to move restored data")?;

        Ok(())
    }
}
