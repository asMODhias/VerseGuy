use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{Aead, generic_array::GenericArray},
};
use anyhow::{Context, Result};
use chrono::Utc;
use filetime::FileTime;
use flate2::Compression;
use flate2::write::GzEncoder;
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs::{self, File};
use std::io::{Read, Write};
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

    /// Create a backup. If `encryption_key` is Some(32 bytes), the resulting file will be encrypted
    /// and the returned path will have `.enc` suffix.
    pub fn create_backup(&self, encryption_key: Option<&[u8]>) -> Result<PathBuf> {
        fs::create_dir_all(&self.backup_dir).context("Failed to create backup directory")?;
        let ts = Utc::now().format("%Y%m%d_%H%M%S_%f").to_string();
        let backup_id = format!("backup_{}", ts);
        let filename = format!("{}.tar.gz", backup_id);
        let backup_path = self.backup_dir.join(&filename);

        let tar_gz = File::create(&backup_path).context("Failed to create backup file")?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(enc);
        tar.append_dir_all("data", &self.db_path)
            .context("Failed to append db dir to tar")?;
        tar.finish().context("Failed to finish tar builder")?;
        // ensure all writers are flushed and file is closed before reading it for encryption
        drop(tar);

        if let Some(key) = encryption_key {
            let enc_name = if let Some(fname) = backup_path.file_name() {
                format!("{}.enc", fname.to_string_lossy())
            } else {
                "backup.enc".to_string()
            };
            let enc_path = backup_path.with_file_name(enc_name);
            encrypt_file(&backup_path, &enc_path, key)?;
            fs::remove_file(&backup_path).ok();
            Ok(enc_path)
        } else {
            Ok(backup_path)
        }
    }

    /// Restore backup. If the file ends with `.enc`, `encryption_key` must be provided.
    pub fn restore_backup<P: AsRef<Path>>(
        &self,
        backup_path: P,
        restore_to: P,
        encryption_key: Option<&[u8]>,
    ) -> Result<()> {
        let backup_path = backup_path.as_ref();
        let restore_to = restore_to.as_ref();
        if !backup_path.exists() {
            anyhow::bail!("Backup not found: {:?}", backup_path);
        }

        let mut effective_path = PathBuf::from(backup_path);
        let mut _temp_decrypt_dir: Option<tempfile::TempDir> = None;
        if backup_path.extension().and_then(|s| s.to_str()) == Some("enc") {
            let key = encryption_key
                .ok_or_else(|| anyhow::anyhow!("Encryption key required for .enc backups"))?;
            let dec_tmp = tempfile::tempdir().context("Failed to create tempdir")?;
            let dec_path = dec_tmp.path().join("decrypted.tar.gz");
            decrypt_file(backup_path, &dec_path, key)?;
            effective_path = dec_path;
            _temp_decrypt_dir = Some(dec_tmp);
        }

        // create temp dir for unpacking
        let unpack_tmp = tempfile::tempdir().context("Failed to create tempdir")?;
        let mut tar_gz = File::open(&effective_path).context("Failed to open backup file")?;
        let mut buf = Vec::new();
        tar_gz.read_to_end(&mut buf)?;
        let dec = flate2::read::GzDecoder::new(&buf[..]);
        let mut archive = tar::Archive::new(dec);
        archive
            .unpack(unpack_tmp.path())
            .context("Failed to unpack archive")?;

        // replace restore_to with extracted data/data
        let extracted = unpack_tmp.path().join("data");
        if restore_to.exists() {
            fs::remove_dir_all(restore_to).context("Failed to remove existing db path")?;
        }
        fs::rename(&extracted, restore_to).context("Failed to move restored data")?;

        // update file times on backups to now so retention rules can operate predictably when used in tests
        let now = FileTime::now();
        filetime::set_file_mtime(&effective_path, now).ok();

        // temp_decrypted will be auto-cleaned when tempdir goes out of scope
        Ok(())
    }

    /// Cleanup old backups, keeping `keep` newest files (by modified time). Returns number deleted.
    pub fn cleanup_old_backups(&self, keep: usize) -> Result<usize> {
        let mut entries: Vec<_> = fs::read_dir(&self.backup_dir)
            .context("Failed to read backup directory")?
            .filter_map(|e| e.ok())
            .filter(|e| {
                if let Some(n) = e.file_name().to_str() {
                    n.starts_with("backup_")
                } else {
                    false
                }
            })
            .collect();

        entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
        entries.reverse();

        if entries.len() <= keep {
            return Ok(0);
        }
        let mut deleted = 0usize;
        for e in entries.iter().skip(keep) {
            if fs::remove_file(e.path()).is_ok() {
                deleted += 1;
            }
        }
        Ok(deleted)
    }
}

/// Encrypts `input` file into `output` using AES-256-GCM.
fn encrypt_file(input: &Path, output: &Path, key: &[u8]) -> Result<()> {
    let mut indata = Vec::new();
    File::open(input)?.read_to_end(&mut indata)?;
    let k = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(k);
    let mut nonce_bytes = [0u8; 12];
    let mut rng = OsRng;
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, indata.as_ref())
        .map_err(|e| anyhow::anyhow!("encryption failed: {}", e))?;
    let mut out = File::create(output)?;
    out.write_all(&nonce_bytes)?;
    out.write_all(&ciphertext)?;
    Ok(())
}

/// Decrypts `input` (contains nonce + ciphertext) into `output`.
fn decrypt_file(input: &Path, output: &Path, key: &[u8]) -> Result<()> {
    let mut indata = Vec::new();
    File::open(input)?.read_to_end(&mut indata)?;
    if indata.len() < 12 {
        anyhow::bail!("invalid encrypted file")
    }
    let (nonce_bytes, ctext) = indata.split_at(12);
    let k = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(k);
    let nonce = GenericArray::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ctext)
        .map_err(|e| anyhow::anyhow!("decryption failed: {}", e))?;
    let mut out = File::create(output)?;
    out.write_all(&plaintext)?;
    Ok(())
}
