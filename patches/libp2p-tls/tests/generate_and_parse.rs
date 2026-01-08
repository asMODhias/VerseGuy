use libp2p_identity as identity;

use libp2p_tls as tls;
use x509_parser::prelude::FromDer;

#[test]
fn generate_certificate_and_parse_extension() {
    // Generate identity keypair (ed25519)
    let keypair = identity::Keypair::generate_ed25519();

    // Generate certificate & key (these will include the libp2p TLS extension signed by keypair)
    let (cert_der, _key_der) = match tls::certificate::generate(&keypair) {
        Ok(v) => v,
        Err(e) => panic!("generate: {:?}", e),
    };

    // Parse the certificate and ensure SPKI and signature exist
    let p2p_cert = match tls::certificate::parse(&cert_der) {
        Ok(p) => p,
        Err(e) => panic!("parse: {:?}", e),
    };
    assert!(p2p_cert.spki.is_some(), "SPKI should be present in extension");
    assert!(p2p_cert.signature.is_some(), "signature should be present in extension");

    // If spki/signature present, try to verify signature using the public key
    let spki = match p2p_cert.spki {
        Some(s) => s,
        None => panic!("spki missing"),
    };
    let sig = match p2p_cert.signature {
        Some(s) => s,
        None => panic!("signature missing"),
    };

    // Message is prefix || spki
    let mut msg = Vec::new();
    msg.extend_from_slice(b"libp2p:tls:");
    msg.extend_from_slice(&spki);

    // Build ed25519 public key from SPKI (subject_public_key bitstring in DER contains raw public key)
    let (_, spki_struct) = match x509_parser::x509::SubjectPublicKeyInfo::from_der(&spki) {
        Ok(s) => s,
        Err(e) => panic!("spki parse: {:?}", e),
    };
    let pk_bits = spki_struct.subject_public_key.data;

    // Verify signature with the identity keypair's public key (we used `keypair` to sign the extension)
    assert!(keypair.public().verify(&msg, &sig), "identity public key should verify the signature");
}


