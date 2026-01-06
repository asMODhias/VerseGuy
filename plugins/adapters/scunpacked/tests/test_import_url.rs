use plugins_adapter_scunpacked::import_from_url;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use verseguy_test_utils::must;
use tempfile::tempdir;
use verseguy_storage::RocksDBStorage;

fn serve_once(port: u16, body: &'static str) {
    let listener = must(TcpListener::bind(("127.0.0.1", port)));
    // accept one connection
    if let Ok((mut stream, _)) = listener.accept() {
        let mut req = [0u8; 1024];
        let _ = stream.read(&mut req);
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    }
}

#[test]
fn test_import_from_url_server() {
    let port = 18080;
    let body = r#"[{"id":"u-a","name":"U A","role":"test"}]"#;
    let handle = thread::spawn(move || serve_once(port, body));

    let dir = must(tempdir());
    let storage = must(RocksDBStorage::open(dir.path().join("db")));

    let url = format!("http://127.0.0.1:{}", port);
    let n = must(import_from_url(&url, &storage, None));
    assert_eq!(n, 1);

    match handle.join() {
        Ok(_) => {}
        Err(e) => panic!("thread join failed: {:?}", e),
    }
} 
