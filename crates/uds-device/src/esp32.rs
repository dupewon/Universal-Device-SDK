use crate::traits::{Device, DeviceError};
use uds_core::{DeviceId, DeviceInfo, DeviceStatus, DeviceCapabilitySet, TransportType, TransportHint,
                HardwareCapabilities, FirmwareCapabilities, FeatureFlags};
use std::fmt;

#[derive(Debug)]
pub struct Esp32Device {
    id: DeviceId,
    info: DeviceInfo,
    status: DeviceStatus,
}

impl Esp32Device {
    pub fn new(id: &str, transport_addr: &str, transport_type: TransportType) -> Self {
        Self {
            id: DeviceId(id.to_string()),
            info: DeviceInfo {
                id: DeviceId(id.to_string()),
                name: format!("ESP32 ({})", id),
                transport_hints: vec![TransportHint {
                    transport_type,
                    address: transport_addr.to_string(),
                }],
                capabilities: DeviceCapabilitySet {
                    hardware: HardwareCapabilities {
                        cpu_type: "Xtensa LX6".into(),
                        flash_size_kb: 4096,
                        ram_size_kb: 520,
                        has_radio: true,
                    },
                    firmware: FirmwareCapabilities {
                        version: "0.1.0".into(),
                        protocol_version_major: 1,
                        protocol_version_minor: 0,
                        supports_streaming: true,
                        supports_compression: false,
                        supports_encryption: true,
                    },
                    features: FeatureFlags {
                        ota: true, filesystem: true, logging: true, monitoring: true, benchmarking: true,
                    },
                },
            },
            status: DeviceStatus::Disconnected,
        }
    }

    pub fn probe_at(port: &str, baud: u32) -> Option<Self> {
        tracing::info!("Probing ESP32 at {} @ {} baud...", port, baud);
        Some(Self::new(
            &format!("esp32-{}", port.replace(|c: char| !c.is_alphanumeric(), "")),
            port,
            TransportType::Serial,
        ))
    }
}

impl Device for Esp32Device {
    fn id(&self) -> &DeviceId { &self.id }
    fn info(&self) -> &DeviceInfo { &self.info }
    fn capabilities(&self) -> &DeviceCapabilitySet { &self.info.capabilities }
    fn status(&self) -> DeviceStatus { self.status }

    fn connect(&self) -> Result<(), DeviceError> {
        tracing::info!("Connecting to ESP32: {}", self.id.0);
        Ok(())
    }

    fn disconnect(&self) -> Result<(), DeviceError> {
        tracing::info!("Disconnecting ESP32: {}", self.id.0);
        Ok(())
    }

    fn reset(&self) -> Result<(), DeviceError> {
        tracing::info!("Resetting ESP32: {}", self.id.0);
        Ok(())
    }

    fn flash(&self, image: &[u8]) -> Result<(), DeviceError> {
        tracing::info!("Flashing {} bytes to ESP32: {}", image.len(), self.id.0);
        Ok(())
    }

    fn send_rpc(&self, method: &str, params: &[u8]) -> Result<Vec<u8>, DeviceError> {
        tracing::debug!("RPC call on {}: {} ({} bytes)", self.id.0, method, params.len());
        Ok(b"ok".to_vec())
    }
}
