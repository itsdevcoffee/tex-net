use anyhow::Context;
use quinn::{Endpoint, ServerConfig};
use rustls::{Certificate, PrivateKey};
use std::{error::Error, fs, net::SocketAddr, sync::Arc};

pub struct TexServer {
    pub cert_path: String,
    pub key_path: String,
}

impl Default for TexServer {
    fn default() -> Self {
        TexServer {
            cert_path: String::from("cert/localhost.crt"), // Provide default cert path
            key_path: String::from("cert/localhost.key"),  // Provide default key path
        }
    }
}

impl TexServer {
    /// Returns default server configuration along with its certificate.
    pub fn configure_server(&self) -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
        // Read the private key and certificate from file
        let cert_file = fs::read(self.cert_path.to_string()).context("failed to read cert")?;
        let key_file = fs::read(self.key_path.to_string()).context("failed to read private key")?;

        let mut cert_reader = std::io::Cursor::new(cert_file.clone());
        let certs =
            rustls_pemfile::certs(&mut cert_reader).map_err(|_| "Failed to read cert file")?;
        if certs.is_empty() {
            return Err("No certs found".into());
        }

        // Deserialize the certificate and private key
        let cert = Certificate(certs[0].clone());

        // Parse the private key
        let mut key_reader = std::io::Cursor::new(key_file);
        let key_pem = rustls_pemfile::ec_private_keys(&mut key_reader)
            .map_err(|_| "Failed to read private key")?;

        if key_pem.is_empty() {
            return Err("No private keys found".into());
        }

        let priv_key = PrivateKey(key_pem[0].clone());

        // Create the server configuration
        let mut server_config =
            quinn::ServerConfig::with_single_cert(vec![cert.clone()], priv_key)?;

        // Configure the transport parameters as needed
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());

        // Return the server configuration and the certificate
        Ok((server_config, cert.0))
    }

    pub fn make_server_endpoint(
        &self,
        bind_addr: SocketAddr,
    ) -> Result<(Endpoint, Vec<u8>), Box<dyn Error>> {
        let (server_config, server_cert) = self.configure_server()?;
        let endpoint = Endpoint::server(server_config, bind_addr)?;
        Ok((endpoint, server_cert))
    }

    /// Runs a QUIC server bound to given address and returns server certificate.
    pub fn run_server(&self, addr: SocketAddr) -> Result<Vec<u8>, Box<dyn Error>> {
        let (endpoint, server_cert) = self.make_server_endpoint(addr)?;
        // accept a single connection
        tokio::spawn(async move {
            let connection = endpoint.accept().await.unwrap().await.unwrap();
            println!(
                "[server] incoming connection: addr={}",
                connection.remote_address()
            );
        });

        Ok(server_cert)
    }
}
