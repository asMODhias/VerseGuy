use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Start a simple echo server that responds to a single "ping" with "pong".
/// Returns the listen address (e.g. "127.0.0.1:12345") and a JoinHandle (task).
#[allow(dead_code)]
pub async fn start_echo_server() -> Result<String> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
    let addr = listener.local_addr()?;
    let addr_str = addr.to_string();

    // spawn background task to serve a single connection (keeps running in tests)
    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((mut socket, _peer)) => {
                    let mut buf = [0u8; 1024];
                    match socket.read(&mut buf).await {
                        Ok(n) if n > 0 => {
                            let req = &buf[..n];
                            if req == b"ping" {
                                let _ = socket.write_all(b"pong").await;
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("echo server accept error: {}", e);
                    break;
                }
            }
        }
    });

    Ok(addr_str)
}

/// Ping an address ("127.0.0.1:12345") and return the response as a String.
#[allow(dead_code)]
pub async fn ping_addr(addr: &str) -> Result<String> {
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b"ping").await?;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 16];
    let n = stream.read(&mut tmp).await?;
    buf.extend_from_slice(&tmp[..n]);
    Ok(String::from_utf8_lossy(&buf).to_string())
}
