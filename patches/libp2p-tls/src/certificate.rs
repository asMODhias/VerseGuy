// Local patch of libp2p-tls certificate generation to be compatible with newer rcgen API

use libp2p_identity as identity;
use libp2p_identity::PeerId;
use x509_parser::{prelude::*, signature_algorithm::SignatureAlgorithm};

use std::sync::Arc;

// ... (rest of the file is identical to upstream except for small API adaptions)

use crate::GenError;

pub fn generate(
    identity_keypair: &identity::Keypair,
) -> Result<(
    rustls::pki_types::CertificateDer<'static>,
    rustls::pki_types::PrivateKeyDer<'static>,
), GenError> {
    let certificate_keypair = rcgen::KeyPair::generate_for(P2P_SIGNATURE_ALGORITHM)?;
    let rustls_key = rustls::pki_types::PrivateKeyDer::from(
        rustls::pki_types::PrivatePkcs8KeyDer::from(certificate_keypair.serialize_der()),
    );

    let certificate = {
        let mut params = rcgen::CertificateParams::new(vec![])?;
        params.distinguished_name = rcgen::DistinguishedName::new();
        params.custom_extensions.push(make_libp2p_extension(
            identity_keypair,
            &certificate_keypair,
        )?);
        params.alg = P2P_SIGNATURE_ALGORITHM;
        params.key_pair = Some(certificate_keypair);
        // Use the newer self_signed API
        params.self_signed(&params.key_pair.as_ref().unwrap())?
    };

    let rustls_certificate = rustls::pki_types::CertificateDer::from(certificate.serialize_der()?);

    Ok((rustls_certificate, rustls_key))
}

fn make_libp2p_extension(
    identity_keypair: &identity::Keypair,
    certificate_keypair: &rcgen::KeyPair,
) -> Result<rcgen::CustomExtension, GenError> {
    // ... use public_key_raw instead of public_key_der
    let mut msg = vec![];
    msg.extend(P2P_SIGNING_PREFIX);
    msg.extend(certificate_keypair.public_key_raw());

    let signature = identity_keypair.sign(&msg)?;
    let signed_key = yasna::construct_der(|writer| {
        writer.write_sequence(|writer| {
            writer.next().write_bytes(&certificate_keypair.public_key_raw());
            writer.next().write_bytes(&signature);
        });
    });

    Ok(rcgen::CustomExtension::new(
        P2P_EXT_OID.iter().map(|v| *v).collect(),
        signed_key,
    ))
}
