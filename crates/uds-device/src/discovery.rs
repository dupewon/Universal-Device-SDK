use crate::traits::{DeviceDiscovery, DeviceError, DeviceEvent};
use uds_core::{DeviceInfo, DeviceId, TransportType, TransportHint, DeviceCapabilitySet,
                HardwareCapabilities, FirmwareCapabilities, FeatureFlags};
use std::time::Duration;
use std::sync::mpsc;

pub struct DiscoveryService;

impl DiscoveryService {
    pub fn new() -> Self { Self }

    pub fn scan_serial() -> Vec<String> {
        let mut ports = Vec::new();
        #[cfg(windows)]
        for i in 1..=256 {
            let name = format!("COM{}", i);
            if std::path::Path::new(&format!("\\\\.\\{}", name)).exists() {
                ports.push(name);
            }
        }
        #[cfg(unix)]
        if let Ok(entries) = std::fs::read_dir("/dev") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("ttyUSB") || name.starts_with("ttyACM") || name.starts_with("ttyS") {
                    ports.push(format!("/dev/{}", name));
                }
            }
        }
        ports
    }

    pub fn mock_devices() -> Vec<DeviceInfo> {
        vec![
            DeviceInfo {
                id: DeviceId("esp32-001".into()),
                name: "ESP32 DevKit V1".into(),
                transport_hints: vec![
                    TransportHint { transport_type: TransportType::Serial, address: "COM3".into() },
                    TransportHint { transport_type: TransportType::Tcp, address: "192.168.1.100:4567".into() },
                ],
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
                        ota: true, filesystem: true, logging: true, monitoring: true, benchmarking: false,
                    },
                },
            },
            DeviceInfo {
                id: DeviceId("stm32-001".into()),
                name: "STM32F407 Discovery".into(),
                transport_hints: vec![
                    TransportHint { transport_type: TransportType::Serial, address: "COM5".into() },
                ],
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
                        ota: true, filesystem: true, logging: true, monitoring: true, benchmarking: false,
                    },
                },
            },
            DeviceInfo {
                id: DeviceId("rp2040-001".into()),
                name: "Raspberry Pi Pico".into(),
                transport_hints: vec![
                    TransportHint { transport_type: TransportType::Usb, address: "USB:2.1".into() },
                ],
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
                        ota: true, filesystem: false, logging: true, monitoring: false, benchmarking: false,
                    },
                },
            },
        ]
    }
}

impl DeviceDiscovery for DiscoveryService {
    fn discover(&self, transport: &str, _timeout: Duration) -> Result<Vec<DeviceInfo>, DeviceError> {
        match transport {
            "serial" => {
                let ports = Self::scan_serial();
                if ports.is_empty() {
                    Ok(Self::mock_devices())
                } else {
                    Ok(ports.into_iter().map(|p| DeviceInfo {
                        id: DeviceId(format!("device-{}", p)),
                        name: format!("Device on {}", p),
                        transport_hints: vec![
                            TransportHint { transport_type: TransportType::Serial, address: p },
                        ],
                        capabilities: DeviceCapabilitySet {
                            hardware: HardwareCapabilities {
                                cpu_type: "unknown".into(), flash_size_kb: 0, ram_size_kb: 0, has_radio: false,
                            },
                            firmware: FirmwareCapabilities {
                                version: "unknown".into(), protocol_version_major: 1, protocol_version_minor: 0,
                                supports_streaming: false, supports_compression: false, supports_encryption: false,
                            },
                            features: FeatureFlags {
                                ota: false, filesystem: false, logging: true, monitoring: false, benchmarking: false,
                            },
                        },
                    }).collect())
                }
            }
            "tcp" | "udp" => {
                Ok(Self::mock_devices())
            }
            _ => Ok(Self::mock_devices()),
        }
    }

    fn watch(&self, callback: Box<dyn Fn(DeviceEvent) + Send>) -> Result<(), DeviceError> {
        let devices = Self::mock_devices();
        for d in devices {
            callback(DeviceEvent::Discovered(d));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_mock() {
        let service = DiscoveryService::new();
        let devices = service.discover("mock", Duration::from_secs(1)).unwrap();
        assert!(!devices.is_empty());
        assert!(devices.iter().any(|d| d.id.0.contains("esp32")));
    }

    #[test]
    fn test_serial_ports() {
        let ports = DiscoveryService::scan_serial();
        // Should not crash on any platform
        println!("Serial ports found: {:?}", ports);
    }
}
