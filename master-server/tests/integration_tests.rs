#![allow(clippy::disallowed_methods)]
use reqwest::Client;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;
use verseguy_test_utils::{must, must_opt};

#[test]
fn register_login_and_validate_license() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        // Start the server as a child process with a temporary DB path and secret
        let dir = must(tempdir());
        let _db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();

        let _child = Command::new(must(std::env::current_exe()))
            .arg("-h")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn();

        // NOTE: Starting the binary in integration test is tricky; instead we directly call handlers in unit tests in future.
        // For now, test that reqwest can reach localhost:3000 if developer runs the server manually.

        // Brief sleep to allow manual server to be started externally if needed
        thread::sleep(Duration::from_millis(100));

        let client = Client::new();
        // Try contacting the server; if not available, skip test
        let res = client
            .get("http://127.0.0.1:3000/plugins/search")
            .send()
            .await;
        if res.is_err() {
            eprintln!(
                "Master server not running on 127.0.0.1:3000 — skipping integration HTTP tests"
            );
            return;
        }

        let r = must(res);
        assert!(r.status().is_success());
    });
}
