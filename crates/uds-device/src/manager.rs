use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::traits::{Device, DeviceError, DeviceManager};
use uds_core::{DeviceId, DeviceInfo, DeviceStatus};
use std::fmt;

struct DeviceWrapper(Arc<dyn Device>);

impl Device for DeviceWrapper {
    fn id(&self) -> &DeviceId { self.0.id() }
    fn info(&self) -> &DeviceInfo { self.0.info() }
    fn status(&self) -> DeviceStatus { self.0.status() }
    fn connect(&self) -> Result<(), DeviceError> { self.0.connect() }
    fn disconnect(&self) -> Result<(), DeviceError> { self.0.disconnect() }
    fn send(&self, data: &[u8]) -> Result<(), DeviceError> { self.0.send(data) }
    fn recv(&self, buf: &mut [u8]) -> Result<usize, DeviceError> { self.0.recv(buf) }
}

#[derive(Clone)]
struct DeviceEntry {
    device: Arc<dyn Device>,
}

impl fmt::Debug for DeviceEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceEntry")
            .field("id", self.device.id())
            .field("status", &self.device.status())
            .finish()
    }
}

pub struct DeviceManagerImpl {
    devices: Mutex<HashMap<String, DeviceEntry>>,
}

impl DeviceManagerImpl {
    pub fn new() -> Self {
        Self { devices: Mutex::new(HashMap::new()) }
    }
}

impl DeviceManager for DeviceManagerImpl {
    fn register(&self, device: Box<dyn Device>) {
        let id = device.id().0.clone();
        let mut devices = self.devices.lock().unwrap();
        devices.insert(id, DeviceEntry { device: device.into() });
    }

    fn unregister(&self, id: &DeviceId) {
        let mut devices = self.devices.lock().unwrap();
        devices.remove(&id.0);
    }

    fn get(&self, id: &DeviceId) -> Option<Box<dyn Device>> {
        let devices = self.devices.lock().unwrap();
        devices.get(&id.0).and_then(|entry| {
            Some(Box::new(DeviceWrapper(Arc::clone(&entry.device))))
        })
    }

    fn list(&self) -> Vec<DeviceInfo> {
        let devices = self.devices.lock().unwrap();
        devices.values().map(|d| d.device.info().clone()).collect()
    }

    fn connect_all(&self) -> Vec<Result<(), DeviceError>> {
        let devices = self.devices.lock().unwrap();
        devices.values().map(|d| d.device.connect()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esp32::Esp32Device;
    use uds_core::TransportType;

    #[test]
    fn test_register_and_list() {
        let mgr = DeviceManagerImpl::new();
        let dev = Esp32Device::new("test-1", "/dev/ttyUSB0", TransportType::Serial);
        mgr.register(Box::new(dev));
        assert_eq!(mgr.list().len(), 1);
    }
}
