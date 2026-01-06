pub mod certificate;

pub use certificate::*;

/// Generic error type for certificate generation helper functions
pub type GenError = Box<dyn std::error::Error + Send + Sync + 'static>;

use libp2p_identity as identity;
use rustls::client::ClientConfig as RustlsClientConfig;
use rustls::server::ServerConfig as RustlsServerConfig;
use rustls::RootCertStore;
use std::sync::Arc;

/// Build a `rustls::ClientConfig` using a generated certificate from the provided identity keypair.
/// The returned `ClientConfig` will trust the server certificate generated from the same identity.
pub fn make_client_config(
    identity_keypair: &identity::Keypair,
    _server_name: Option<&str>,
) -> Result<RustlsClientConfig, GenError> {
    let (cert_der, _key_der) = crate::certificate::generate(identity_keypair)?;

    // Build root store trusting the generated cert
    let mut roots = RootCertStore::empty();
    roots.add(cert_der).map_err(|e| Box::new(e) as GenError)?;

    // Build ClientConfig using the config builder and our root store
    let client = RustlsClientConfig::builder()
        .with_root_certificates(Arc::new(roots))
        .with_no_client_auth();

    Ok(client)
}


/// Build a `rustls::ServerConfig` using a generated certificate from the provided identity keypair.
pub fn make_server_config(identity_keypair: &identity::Keypair) -> Result<RustlsServerConfig, GenError> {
    let (cert_der, key_der) = crate::certificate::generate(identity_keypair)?;

    // Server config builder expects DER-encoded cert chain (CertificateDer) and a PrivateKeyDer
    let cert_chain = vec![cert_der];
    let key = key_der;

    let server = RustlsServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .map_err(|e| Box::new(e) as GenError)?;

    Ok(server)
}
