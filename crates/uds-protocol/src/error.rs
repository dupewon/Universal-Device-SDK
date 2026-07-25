use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("frame too short: {0} bytes")]
    FrameTooShort(usize),
    #[error("invalid magic bytes")]
    InvalidMagic,
    #[error("checksum mismatch: expected {expected:04x}, got {got:04x}")]
    ChecksumMismatch { expected: u16, got: u16 },
    #[error("payload too long: {0} bytes (max 65535)")]
    PayloadTooLong(usize),
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(u8),
    #[error("handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("invalid message type: {0}")]
    InvalidMessageType(u8),
    #[error("invalid UTF-8 in message field")]
    InvalidUtf8,
    #[error("protocol not negotiated")]
    NotNegotiated,
    #[error("encryption error: {0}")]
    Encryption(String),
}
