use crate::traits::{Device, DeviceError};
use std::fmt;
use uds_core::{
    DeviceCapabilitySet, DeviceId, DeviceInfo, DeviceStatus, FeatureFlags, FirmwareCapabilities,
    HardwareCapabilities, TransportHint, TransportType,
};

#[derive(Debug)]
pub struct Stm32Device {
    id: DeviceId,
    info: DeviceInfo,
    status: DeviceStatus,
}

impl Stm32Device {
    pub fn new(id: &str, transport_addr: &str, transport_type: TransportType) -> Self {
        Self {
            id: DeviceId(id.to_string()),
            info: DeviceInfo {
                id: DeviceId(id.to_string()),
                name: format!("STM32 ({})", id),
                transport_hints: vec![TransportHint {
                    transport_type,
                    address: transport_addr.to_string(),
                }],
                capabilities: DeviceCapabilitySet {
                    hardware: HardwareCapabilities {
                        cpu_type: "ARM Cortex-M4".into(),
                        flash_size_kb: 2048,
                        ram_size_kb: 256,
                        has_radio: false,
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
                        ota: true,
                        filesystem: true,
                        logging: true,
                        monitoring: true,
                        benchmarking: false,
                    },
                },
                connected: false,
                platform: "STM32".into(),
                firmware_version: "0.1.0".into(),
            },
            status: DeviceStatus::Disconnected,
        }
    }

    pub fn probe_at(port: &str) -> Option<Self> {
        tracing::info!("Probing STM32 at {}...", port);
        Some(Self::new(
            &format!("stm32-{}", port.replace(|c: char| !c.is_alphanumeric(), "")),
            port,
            TransportType::Serial,
        ))
    }
}

impl Device for Stm32Device {
    fn id(&self) -> &DeviceId {
        &self.id
    }
    fn info(&self) -> &DeviceInfo {
        &self.info
    }
    fn capabilities(&self) -> &DeviceCapabilitySet {
        &self.info.capabilities
    }
    fn status(&self) -> DeviceStatus {
        self.status
    }
    fn connect(&self) -> Result<(), DeviceError> {
        tracing::info!("Connecting to STM32: {}", self.id.0);
        Ok(())
    }
    fn disconnect(&self) -> Result<(), DeviceError> {
        tracing::info!("Disconnecting STM32: {}", self.id.0);
        Ok(())
    }
    fn reset(&self) -> Result<(), DeviceError> {
        tracing::info!("Resetting STM32: {}", self.id.0);
        Ok(())
    }
    fn flash(&self, image: &[u8]) -> Result<(), DeviceError> {
        tracing::info!("Flashing {} bytes to STM32: {}", image.len(), self.id.0);
        Ok(())
    }
    fn send_rpc(&self, method: &str, params: &[u8]) -> Result<Vec<u8>, DeviceError> {
        tracing::debug!(
            "RPC call on {}: {} ({} bytes)",
            self.id.0,
            method,
            params.len()
        );
        Ok(b"ok".to_vec())
    }
}
