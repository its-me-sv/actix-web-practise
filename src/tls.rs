use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use std::fs::File;
use std::io::BufReader;

pub fn get_tls_config() -> rustls::ServerConfig {
    let cert_file =
        &mut BufReader::new(File::open("certificates/cert.pem").expect("cert file not found"));
    let key_file =
        &mut BufReader::new(File::open("certificates/key.pem").expect("key file not found"));

    let cert_chain = certs(cert_file)
        .expect("Unable to read cert file")
        .into_iter()
        .map(Certificate)
        .collect();
    let pvt_key = PrivateKey(
        pkcs8_private_keys(key_file)
            .expect("Unable to read key file")
            .get(0)
            .expect("Private key empty")
            .to_owned(),
    );

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, pvt_key)
        .expect("bad certificate/key")
}
