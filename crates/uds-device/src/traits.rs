use std::fmt;
use std::time::Duration;
use uds_core::{DeviceCapabilitySet, DeviceId, DeviceInfo, DeviceStatus};

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("device not found: {0}")]
    NotFound(String),
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    #[error("operation failed: {0}")]
    OperationFailed(String),
    #[error("timeout")]
    Timeout,
    #[error("not supported")]
    NotSupported,
    #[error("authentication failed: {0}")]
    AuthFailed(String),
}

pub trait Device: Send + Sync + fmt::Debug {
    fn id(&self) -> &DeviceId;
    fn info(&self) -> &DeviceInfo;
    fn capabilities(&self) -> &DeviceCapabilitySet;
    fn status(&self) -> DeviceStatus;
    fn connect(&self) -> Result<(), DeviceError>;
    fn disconnect(&self) -> Result<(), DeviceError>;
    fn reset(&self) -> Result<(), DeviceError>;
    fn flash(&self, image: &[u8]) -> Result<(), DeviceError>;
    fn send_rpc(&self, method: &str, params: &[u8]) -> Result<Vec<u8>, DeviceError>;
}

pub trait DeviceDiscovery: Send + Sync {
    fn discover(&self, transport: &str, timeout: Duration) -> Result<Vec<DeviceInfo>, DeviceError>;
    fn watch(&self, callback: Box<dyn Fn(DeviceEvent) + Send>) -> Result<(), DeviceError>;
}

#[derive(Debug, Clone)]
pub enum DeviceEvent {
    Discovered(DeviceInfo),
    Lost(DeviceId),
    Updated(DeviceInfo),
}

pub trait DeviceManager: Send + Sync {
    fn register(&self, device: Box<dyn Device>);
    fn unregister(&self, id: &DeviceId);
    fn get(&self, id: &DeviceId) -> Option<Box<dyn Device>>;
    fn list(&self) -> Vec<DeviceInfo>;
    fn connect_all(&self) -> Vec<Result<(), DeviceError>>;
}
