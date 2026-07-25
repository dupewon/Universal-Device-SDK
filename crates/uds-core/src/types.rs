use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: DeviceId,
    pub name: String,
    pub transport_hints: Vec<TransportHint>,
    pub capabilities: DeviceCapabilitySet,
    pub connected: bool,
    pub platform: String,
    pub firmware_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportHint {
    pub transport_type: TransportType,
    pub address: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportType {
    Serial,
    Tcp,
    Udp,
    WebSocket,
    Ble,
    Usb,
    Mock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilitySet {
    pub hardware: HardwareCapabilities,
    pub firmware: FirmwareCapabilities,
    pub features: FeatureFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    pub cpu_type: String,
    pub flash_size_kb: u32,
    pub ram_size_kb: u32,
    pub has_radio: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareCapabilities {
    pub version: String,
    pub protocol_version_major: u8,
    pub protocol_version_minor: u8,
    pub supports_streaming: bool,
    pub supports_compression: bool,
    pub supports_encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub ota: bool,
    pub filesystem: bool,
    pub logging: bool,
    pub monitoring: bool,
    pub benchmarking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceStatus {
    Disconnected,
    Connected,
    Busy,
    Error,
    Updating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
