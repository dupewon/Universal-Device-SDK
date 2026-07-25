use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("path not found: {0}")]
    NotFound(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("I/O error: {0}")]
    Io(String),
}
