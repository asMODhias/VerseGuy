use std::env;
use std::process;
use std::sync::Arc;
use chrono::Utc;
use verseguy_storage_infra::config::StorageConfig;
use verseguy_storage_infra::engine::StorageEngine;
use verseguy_audit_infra::AuditStore;

fn print_usage() {
    eprintln!("Usage: retention_runner --db-path <path> [--days <days>] [--dry-run]");
}

fn main() {
    let mut args = env::args().skip(1);
    let mut db_path: Option<String> = None;
    let mut days: i64 = 30; // default
    let mut dry_run = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--db-path" => {
                if let Some(p) = args.next() {
                    db_path = Some(p);
                } else {
                    print_usage();
                    process::exit(2);
                }
            }
            "--days" => {
                if let Some(d) = args.next() {
                    days = d.parse().unwrap_or_else(|_| {
                        eprintln!("Invalid days value: {}", d);
                        process::exit(2);
                    });
                } else {
                    print_usage();
                    process::exit(2);
                }
            }
            "--dry-run" => {
                dry_run = true;
            }
            "--help" | "-h" => {
                print_usage();
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown arg: {}", arg);
                print_usage();
                process::exit(2);
            }
        }
    }

    let db_path = match db_path.or_else(|| env::var("AUDIT_DB_PATH").ok()) {
        Some(p) => p,
        None => {
            eprintln!("Missing --db-path or AUDIT_DB_PATH env var");
            print_usage();
            process::exit(2);
        }
    };

    println!("Retention runner starting: db={} days={} dry_run={}", db_path, days, dry_run);

    let cfg = StorageConfig {
        path: std::path::PathBuf::from(db_path),
        encryption_enabled: false,
        ..Default::default()
    };

    let engine = match StorageEngine::open(cfg) {
        Ok(e) => Arc::new(e),
        Err(err) => {
            eprintln!("Failed to open storage engine: {}", err);
            process::exit(1);
        }
    };

    let store = AuditStore::new(engine.clone());

    if dry_run {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        match store.repo.find(|e| e.timestamp < cutoff) {
            Ok(list) => {
                println!("Dry-run: would delete {} events older than {} days", list.len(), days);
            }
            Err(e) => {
                eprintln!("Error enumerating old events: {}", e);
                process::exit(1);
            }
        }
        return;
    }

    match store.run_retention_days(days) {
        Ok(count) => println!("Retention completed: deleted {} events", count),
        Err(err) => {
            eprintln!("Retention failed: {}", err);
            process::exit(1);
        }
    }
}
