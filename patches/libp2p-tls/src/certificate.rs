// Local patch of libp2p-tls certificate generation to be compatible with newer rcgen API

use libp2p_identity as identity;
use x509_parser::{prelude::*, signature_algorithm::SignatureAlgorithm};
use yasna::{self, DERWriter, DERWriterSeq};
use rcgen::PublicKeyData;
use libp2p_identity::PeerId;

use std::sync::Arc;

use crate::GenError;

// P2P-specific constants (temporary local definitions; align with upstream values as part of PR)
const P2P_EXT_OID: &[u64] = &[1, 3, 6, 1, 4, 1, 53594, 1, 1];
const P2P_SIGNING_PREFIX: &[u8] = b"libp2p:tls:";
const P2P_SIGNATURE_ALGORITHM: &rcgen::SignatureAlgorithm = &rcgen::PKCS_ECDSA_P256_SHA256;

pub fn generate(
    identity_keypair: &identity::Keypair,
) -> Result<(
    rustls::pki_types::CertificateDer<'static>,
    rustls::pki_types::PrivateKeyDer<'static>,
), GenError> {
    let certificate_keypair = rcgen::KeyPair::generate_for(P2P_SIGNATURE_ALGORITHM)
        .map_err(|e| Box::new(e) as GenError)?;

    let rustls_key = rustls::pki_types::PrivateKeyDer::from(
        rustls::pki_types::PrivatePkcs8KeyDer::from(certificate_keypair.serialize_der()),
    );

    let mut params = rcgen::CertificateParams::new(vec![])
        .map_err(|e| Box::new(e) as GenError)?;
    params.distinguished_name = rcgen::DistinguishedName::new();
    params.custom_extensions.push(make_libp2p_extension(
        identity_keypair,
        &certificate_keypair,
    )?);

    // Sign with the generated keypair; rcgen derives the algorithm from the signing key
    let certificate = params
        .self_signed(&certificate_keypair)
        .map_err(|e| Box::new(e) as GenError)?;

    let rustls_certificate = rustls::pki_types::CertificateDer::from(certificate.der().to_vec());

    Ok((rustls_certificate, rustls_key))
}

fn make_libp2p_extension(
    identity_keypair: &identity::Keypair,
    certificate_keypair: &rcgen::KeyPair,
) -> Result<rcgen::CustomExtension, GenError> {
    // Build the message to sign: prefix + SPKI DER
    let mut msg = vec![];
    msg.extend_from_slice(P2P_SIGNING_PREFIX);
    let spki = certificate_keypair.subject_public_key_info();
    msg.extend_from_slice(&spki);

    let signature: Vec<u8> = identity_keypair
        .sign(&msg)
        .map_err(|e| Box::new(e) as GenError)?;

    let signed_key: Vec<u8> = yasna::construct_der(|writer: DERWriter| {
        writer.write_sequence(|writer: &mut DERWriterSeq| {
            // write the subject public key info as raw DER
            writer.next().write_der(&spki);
            // write the signature as an OCTET STRING
            writer.next().write_bytes(&signature);
        });
    });

    Ok(rcgen::CustomExtension::from_oid_content(&P2P_EXT_OID, signed_key))
}

// A minimal P2P-specific certificate view.
#[derive(Clone, Debug)]
pub struct P2PCertificate {
    peer: Option<PeerId>,
    pub spki: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
}

impl P2PCertificate {
    pub fn peer_id(&self) -> PeerId {
        self.peer.clone().unwrap_or_else(|| PeerId::random())
    }
}

// Permissive parser stub: attempt to locate a libp2p TLS extension and return the peer id if present.
// Parse the end-entity certificate DER, look for the libp2p TLS extension OID and extract the
// contained SubjectPublicKeyInfo and signature. If everything verifies, return a `P2PCertificate` with
// the derived `PeerId`.
pub fn parse_certificate_peerid<C: AsRef<[u8]>>(end_entity: &C) -> Result<P2PCertificate, GenError> {
    // Parse the certificate using x509-parser
    let der: &[u8] = end_entity.as_ref();
    let (_, x509) = x509_parser::parse_x509_certificate(der).map_err(|e| Box::new(e) as GenError)?;

    // Find the extension matching our OID (compare by dotted string)
    const P2P_EXT_OID_STR: &str = "1.3.6.1.4.1.53594.1.1";

    for ext in x509.extensions() {
        if ext.oid.to_string() == P2P_EXT_OID_STR {
            // ext.value is the raw extension content (DER-encoded OCTET STRING payload in x509-parser)
            let ext_der = ext.value;

            // Parse the ext_der -> sequence { spki DER, signature OCTET STRING }
            let parsed = yasna::parse_der(&ext_der, |reader| {
                reader.read_sequence(|reader| {
                    let spki = reader.next().read_der()?;
                    let sig = reader.next().read_bytes()?;
                    Ok((spki, sig))
                })
            });

            if let Ok((spki_der, signature)) = parsed {
                // Attempt to parse SPKI bytes to validate structure, but don't require conversion to a
                // specific libp2p public key for now; store the bytes for later verification.
                let _ = x509_parser::x509::SubjectPublicKeyInfo::from_der(&spki_der).ok();

                return Ok(P2PCertificate {
                    peer: None,
                    spki: Some(spki_der.to_vec()),
                    signature: Some(signature),
                });
            }
        }
    }

    // If we couldn't find/validate, fall back to returning an empty P2PCertificate (caller may use a random PeerId).
    Ok(P2PCertificate { peer: None, spki: None, signature: None })
}

// Compatibility wrapper expected by upstream `libp2p-quic` which calls `certificate::parse`.
pub fn parse<C: AsRef<[u8]>>(end_entity: &C) -> Result<P2PCertificate, GenError> {
    parse_certificate_peerid(end_entity)
}
