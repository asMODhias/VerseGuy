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
            client.write_tls(&mut client_buf).unwrap();
        }
        // server -> write
        while server.wants_write() {
            server.write_tls(&mut server_buf).unwrap();
        }

        // feed client -> server
        if !client_buf.is_empty() {
            let mut rd = &client_buf[..];
            server.read_tls(&mut rd).unwrap();
            server.process_new_packets().unwrap();
            client_buf.clear();
        }

        // feed server -> client
        if !server_buf.is_empty() {
            let mut rd = &server_buf[..];
            client.read_tls(&mut rd).unwrap();
            client.process_new_packets().unwrap();
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

    let server_cfg = tls::make_server_config(&keypair).expect("server cfg");
    let client_cfg = tls::make_client_config(&keypair, Some("localhost")).expect("client cfg");

    let server_conn = ServerConnection::new(Arc::new(server_cfg)).expect("server conn");
    let client_conn = ClientConnection::new(Arc::new(client_cfg), "localhost".try_into().unwrap()).expect("client conn");

    do_handshake(client_conn, server_conn);
}