use clap::Parser;
use master_server::admin_cli::{Cli, Commands};

#[test]
fn parse_key_import_b64() {
    let args = vec!["verseguy-admin", "--server", "http://127.0.0.1:3000", "key-import", "--b64", "AAA"];
    let cli = Cli::parse_from(args);
    match cli.cmd {
        Commands::KeyImport { file: _, b64 } => assert_eq!(b64.unwrap(), "AAA".to_string()),
        _ => panic!("unexpected cmd"),
    }
}
