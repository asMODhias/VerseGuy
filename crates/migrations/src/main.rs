use anyhow::Result;
use clap::{Parser, Subcommand};
use verseguy_migrations::{Migration, MigrationManager};
use verseguy_storage::RocksDBStorage as Storage;

#[derive(Parser, Debug)]
#[command(name = "verseguy-migrate")]
struct Cli {
    /// Path to RocksDB storage
    #[arg(short, long, default_value = "./data/db")]
    db_path: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Apply pending migrations
    Apply,
    /// Rollback last migration
    Rollback,
    /// List applied migrations
    List,
}

fn built_in_migrations() -> Vec<Migration> {
    fn up(s: &Storage) -> Result<()> {
        s.put("migr:welcome", &"1")?;
        Ok(())
    }
    fn down(s: &Storage) -> Result<()> {
        s.delete("migr:welcome")?;
        Ok(())
    }
    vec![Migration::new(1, "welcome", up, Some(down))]
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let storage = Storage::open(&cli.db_path)?;
    let mgr = MigrationManager::new(built_in_migrations());

    match cli.cmd {
        Commands::Apply => {
            let applied = mgr.apply_pending(&storage)?;
            println!("Applied {} migrations", applied.len());
        }
        Commands::Rollback => {
            let r = mgr.rollback_last(&storage)?;
            match r {
                Some(a) => println!("Rolled back migration {}", a.version),
                None => println!("Nothing to rollback"),
            }
        }
        Commands::List => {
            let ver = mgr.current_version(&storage)?;
            println!("Current version: {}", ver);
        }
    }

    Ok(())
}
