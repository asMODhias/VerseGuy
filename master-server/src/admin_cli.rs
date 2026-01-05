use base64::Engine;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(name = "verseguy-admin")]
pub struct Cli {
    /// Master server base URL (e.g., http://localhost:3000)
    #[arg(short, long, default_value = "http://127.0.0.1:3000")]
    pub server: String,

    /// Admin token for auth (or set MASTER_ADMIN_TOKEN env)
    #[arg(short, long)]
    pub token: Option<String>,

    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show current master key public info
    KeyList,
    /// Rotate the master key
    KeyRotate,
    /// Import a keypair (provide --file or --b64)
    KeyImport {
        #[arg(short, long)]
        file: Option<String>,
        #[arg(long)]
        b64: Option<String>,
    },
}

pub fn run_from_args<I, T>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(args);
    run(cli)
}

pub fn run_from_std_args() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> anyhow::Result<()> {
    let token = cli
        .token
        .or_else(|| std::env::var("MASTER_ADMIN_TOKEN").ok());
    let client = Client::builder().build()?;

    match cli.cmd {
        Commands::KeyList => {
            let url = format!("{}/admin/keys", cli.server);
            let mut req = client.get(&url);
            if let Some(ref t) = token {
                req = req.header("x-admin-token", t);
            }
            let resp = req.send()?;
            let status = resp.status();
            let txt = resp.text()?;
            println!("[key-list] status: {}", status);
            println!(
                "[key-list] body (truncated 2k): {}",
                &txt.chars().take(2048).collect::<String>()
            );
        }
        Commands::KeyRotate => {
            let url = format!("{}/admin/keys/rotate", cli.server);
            let mut req = client.post(&url);
            if let Some(ref t) = token {
                req = req.header("x-admin-token", t);
            }
            let resp = req.send()?;
            let status = resp.status();
            let txt = resp.text()?;
            println!("[key-rotate] status: {}", status);
            println!(
                "[key-rotate] body (truncated 2k): {}",
                &txt.chars().take(2048).collect::<String>()
            );
        }
        Commands::KeyImport { file, b64 } => {
            let url = format!("{}/admin/keys/import", cli.server);
            let key_b64 = if let Some(f) = file {
                let bytes = std::fs::read(f)?;
                base64::engine::general_purpose::STANDARD.encode(&bytes)
            } else if let Some(b) = b64 {
                b
            } else {
                anyhow::bail!("provide --file or --b64")
            };
            let body = json!({"key_b64": key_b64});
            let mut req = client.post(&url).json(&body);
            if let Some(ref t) = token {
                req = req.header("x-admin-token", t);
            }
            let resp = req.send()?;
            let status = resp.status();
            let txt = resp.text()?;
            println!("[key-import] status: {}", status);
            println!(
                "[key-import] body (truncated 2k): {}",
                &txt.chars().take(2048).collect::<String>()
            );
        }
    }

    Ok(())
}
