use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "manifest-tool")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sign {
        manifest: String,
        out_sig: String,
        out_key: String,
        out_pub: String,
    },
    Verify {
        manifest: String,
        sig: String,
        pubkey: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Sign {
            manifest,
            out_sig,
            out_key,
            out_pub,
        } => {
            master_server::manifest_tool::sign_manifest(&manifest, &out_sig, &out_key, &out_pub)?;
            println!("Signed manifest -> {}", out_sig);
        }
        Commands::Verify {
            manifest,
            sig,
            pubkey,
        } => {
            let ok = master_server::manifest_tool::verify_manifest(&manifest, &sig, &pubkey)?;
            if ok {
                println!("OK");
                std::process::exit(0);
            } else {
                eprintln!("FAIL");
                std::process::exit(2);
            }
        }
    }
    Ok(())
}
