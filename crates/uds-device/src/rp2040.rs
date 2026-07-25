use crate::traits::{Device, DeviceError};
use uds_core::{
    DeviceCapabilitySet, DeviceId, DeviceInfo, DeviceStatus, FeatureFlags, FirmwareCapabilities,
    HardwareCapabilities, TransportHint, TransportType,
};

#[derive(Debug)]
pub struct Rp2040Device {
    id: DeviceId,
    info: DeviceInfo,
    status: DeviceStatus,
}

impl Rp2040Device {
    pub fn new(id: &str, address: &str) -> Self {
        Self {
            id: DeviceId(id.to_string()),
            info: DeviceInfo {
                id: DeviceId(id.to_string()),
                name: format!("RP2040 ({})", id),
                transport_hints: vec![TransportHint {
                    transport_type: TransportType::Usb,
                    address: address.to_string(),
                }],
                capabilities: DeviceCapabilitySet {
                    hardware: HardwareCapabilities {
                        cpu_type: "ARM Cortex-M0+".into(),
                        flash_size_kb: 2048,
                        ram_size_kb: 264,
                        has_radio: false,
                    },
                    firmware: FirmwareCapabilities {
                        version: "0.1.0".into(),
                        protocol_version_major: 1,
                        protocol_version_minor: 0,
                        supports_streaming: false,
                        supports_compression: false,
                        supports_encryption: true,
                    },
                    features: FeatureFlags {
                        ota: true,
                        filesystem: false,
                        logging: true,
                        monitoring: false,
                        benchmarking: false,
                    },
                },
                connected: false,
                platform: "RP2040".into(),
                firmware_version: "0.1.0".into(),
            },
            status: DeviceStatus::Disconnected,
        }
    }
}

impl Device for Rp2040Device {
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
        Ok(())
    }
    fn disconnect(&self) -> Result<(), DeviceError> {
        Ok(())
    }
    fn reset(&self) -> Result<(), DeviceError> {
        Ok(())
    }
    fn flash(&self, image: &[u8]) -> Result<(), DeviceError> {
        tracing::info!("Flashing {} bytes to RP2040 via USB", image.len());
        Ok(())
    }
    fn send_rpc(&self, _method: &str, _params: &[u8]) -> Result<Vec<u8>, DeviceError> {
        Ok(b"ok".to_vec())
    }
}
