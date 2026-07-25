use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("transport error: {0}")]
    Transport(String),

    #[error("protocol error: {0}")]
    Protocol(String),

    #[error("device error: {0}")]
    Device(String),

    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("timeout")]
    Timeout,

    #[error("not connected")]
    NotConnected,

    #[error("unsupported version: {0}")]
    UnsupportedVersion(u8),

    #[error("checksum mismatch")]
    ChecksumMismatch,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(String),
}
