use crate::traits::{Transport, TransportConfig, TransportConnection, TransportError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[derive(Debug)]
struct BleInner {
    open: AtomicBool,
}

#[derive(Debug)]
pub struct BleConnection {
    inner: BleInner,
    mac: String,
}

#[derive(Debug)]
pub struct BleTransport;

impl Default for BleTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl BleTransport {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_devices(timeout: Duration) -> Result<Vec<String>, TransportError> {
        tracing::info!("Scanning for BLE devices (timeout: {:?})...", timeout);
        tracing::warn!("BLE transport requires btleplug crate at runtime");
        Ok(Vec::new())
    }
}

impl Transport for BleTransport {
    fn open(
        &self,
        config: TransportConfig,
    ) -> Result<Box<dyn TransportConnection>, TransportError> {
        match config {
            TransportConfig::Ble { mac, .. } => {
                tracing::info!("BLE transport configured for {}", mac);
                Ok(Box::new(BleConnection {
                    inner: BleInner {
                        open: AtomicBool::new(true),
                    },
                    mac,
                }))
            }
            _ => Err(TransportError::Config("expected BLE config".into())),
        }
    }

    fn name(&self) -> &'static str {
        "ble"
    }
    fn is_available(&self) -> bool {
        cfg!(feature = "ble")
    }
}

impl TransportConnection for BleConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        tracing::debug!("BLE send {} bytes to {}", buf.len(), self.mac);
        Ok(buf.len())
    }

    fn recv(&self, _buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        Err(TransportError::Timeout(Duration::from_secs(30)))
    }

    fn close(&self) -> Result<(), TransportError> {
        self.inner.open.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn is_open(&self) -> bool {
        self.inner.open.load(Ordering::Relaxed)
    }

    fn set_timeout(&self, _timeout: Duration) -> Result<(), TransportError> {
        Ok(())
    }
}
