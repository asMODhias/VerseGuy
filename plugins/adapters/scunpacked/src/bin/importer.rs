use anyhow::Result;
use plugins_adapter_scunpacked::{import_from_file, import_from_p4k, import_from_url};
use std::env;
use std::path::PathBuf;
use verseguy_storage::RocksDBStorage;

fn usage_and_exit() -> ! {
    eprintln!(
        "Usage: importer --file <path> | --url <url> [--db <db_path>] [--if-newer <rfc3339>]"
    );
    std::process::exit(2);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        usage_and_exit();
    }

    let mut src_file: Option<String> = None;
    let mut src_url: Option<String> = None;
    let mut src_p4k: Option<String> = None;
    let mut db_path = PathBuf::from("data/scunpacked/db");
    let mut if_newer: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                src_file = Some(args[i].clone());
            }
            "--url" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                src_url = Some(args[i].clone());
            }
            "--db" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                db_path = PathBuf::from(&args[i]);
            }
            "--if-newer" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                if_newer = Some(args[i].clone());
            }
            "--p4k" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                src_p4k = Some(args[i].clone());
            }
            _ => {
                usage_and_exit();
            }
        }
        i += 1;
    }

    if src_file.is_none() && src_url.is_none() {
        usage_and_exit();
    }

    println!("Opening storage at {}", db_path.display());
    let storage = RocksDBStorage::open(&db_path)?;

    let imported = if let Some(f) = src_file {
        println!("Importing from file: {}", f);
        import_from_file(f, &storage)?
    } else if let Some(u) = src_url {
        println!("Importing from URL: {}", u);
        import_from_url(u, &storage, if_newer.as_deref())?
    } else if let Some(p) = src_p4k {
        println!("Importing from p4k: {}", p);
        import_from_p4k(p, &storage)?
    } else {
        0
    };

    println!("Imported {} items", imported);
    Ok(())
}
