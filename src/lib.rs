// Externals
mod client;
mod server;

pub use client::*;
pub use server::*;

// Internals

/// The HTTP/3 ALPN is required when negotiating a QUIC connection.
pub static ALPN: &[u8] = b"h3";
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
