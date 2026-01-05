use plugins_adapter_scunpacked::import_from_url;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use tempfile::tempdir;
use verseguy_storage::RocksDBStorage;

fn serve_once(port: u16, body: &'static str) {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
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

    let dir = tempdir().unwrap();
    let storage = RocksDBStorage::open(dir.path().join("db")).unwrap();

    let url = format!("http://127.0.0.1:{}", port);
    let n = import_from_url(&url, &storage, None).unwrap();
    assert_eq!(n, 1);
    handle.join().unwrap();
}
