use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("method not found: {0}")]
    MethodNotFound(String),
    #[error("transport error: {0}")]
    Transport(String),
    #[error("protocol error: {0}")]
    Protocol(String),
    #[error("timeout")]
    Timeout,
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("remote error: {0}")]
    RemoteError(String),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("not supported: {0}")]
    NotSupported(String),
}
