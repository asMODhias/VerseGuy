use libp2p_identity as identity;
use libp2p_tls as tls;
use rustls::{client::ClientConnection, server::ServerConnection};
use std::io::{Read, Write};
use std::sync::Arc;

fn do_handshake(mut client: ClientConnection, mut server: ServerConnection) {
    let mut client_buf = Vec::new();
    let mut server_buf = Vec::new();

    // Run a loop that pumps data between the connections until both are not handshaking.
    loop {
        // client -> write
        while client.wants_write() {
            if let Err(e) = client.write_tls(&mut client_buf) { panic!("client write_tls: {:?}", e); }
        }
        // server -> write
        while server.wants_write() {
            if let Err(e) = server.write_tls(&mut server_buf) { panic!("server write_tls: {:?}", e); }
        }

        // feed client -> server
        if !client_buf.is_empty() {
            let mut rd = &client_buf[..];
            if let Err(e) = server.read_tls(&mut rd) { panic!("server read_tls: {:?}", e); }
            if let Err(e) = server.process_new_packets() { panic!("server process_new_packets: {:?}", e); }
            client_buf.clear();
        }

        // feed server -> client
        if !server_buf.is_empty() {
            let mut rd = &server_buf[..];
            if let Err(e) = client.read_tls(&mut rd) { panic!("client read_tls: {:?}", e); }
            if let Err(e) = client.process_new_packets() { panic!("client process_new_packets: {:?}", e); }
            server_buf.clear();
        }

        if !client.is_handshaking() && !server.is_handshaking() {
            break;
        }
    }
}

#[test]
fn tls_handshake_between_client_and_server() {
    let keypair = identity::Keypair::generate_ed25519();

    // Ensure a process-wide CryptoProvider is installed (required by rustls 0.23+ when multiple providers may be available)
    rustls::crypto::CryptoProvider::install_default();

    let server_cfg = match tls::make_server_config(&keypair) {
        Ok(c) => c,
        Err(e) => panic!("server cfg: {:?}", e),
    };
    let client_cfg = match tls::make_client_config(&keypair, Some("localhost")) {
        Ok(c) => c,
        Err(e) => panic!("client cfg: {:?}", e),
    };

    let server_conn = match ServerConnection::new(Arc::new(server_cfg)) {
        Ok(s) => s,
        Err(e) => panic!("server conn: {:?}", e),
    };
    let server_name = match "localhost".try_into() {
        Ok(s) => s,
        Err(e) => panic!("server name conversion: {:?}", e),
    };
    let client_conn = match ClientConnection::new(Arc::new(client_cfg), server_name) {
        Ok(c) => c,
        Err(e) => panic!("client conn: {:?}", e),
    };

    do_handshake(client_conn, server_conn);
}