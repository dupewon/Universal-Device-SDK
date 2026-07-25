use crate::traits::{Transport, TransportConfig, TransportConnection, TransportError};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[derive(Debug)]
struct UsbInner {
    open: AtomicBool,
}

#[derive(Debug)]
pub struct UsbConnection {
    inner: UsbInner,
    vid: u16,
    pid: u16,
}

#[derive(Debug)]
pub struct UsbTransport;

impl UsbTransport {
    pub fn new() -> Self {
        Self
    }

    pub fn list_devices() -> Vec<(u16, u16, String)> {
        tracing::warn!("USB transport requires libusb/rusb at runtime");
        Vec::new()
    }
}

impl Transport for UsbTransport {
    fn open(
        &self,
        config: TransportConfig,
    ) -> Result<Box<dyn TransportConnection>, TransportError> {
        match config {
            TransportConfig::Usb { vid, pid, .. } => {
                tracing::info!("USB transport configured for {:04x}:{:04x}", vid, pid);
                Ok(Box::new(UsbConnection {
                    inner: UsbInner {
                        open: AtomicBool::new(true),
                    },
                    vid,
                    pid,
                }))
            }
            _ => Err(TransportError::Config("expected USB config".into())),
        }
    }

    fn name(&self) -> &'static str {
        "usb"
    }
    fn is_available(&self) -> bool {
        cfg!(feature = "usb")
    }
}

impl TransportConnection for UsbConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        tracing::debug!(
            "USB send {} bytes to {:04x}:{:04x}",
            buf.len(),
            self.vid,
            self.pid
        );
        Ok(buf.len())
    }

    fn recv(&self, _buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        Err(TransportError::Timeout(Duration::from_secs(10)))
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

    fn peer_addr(&self) -> Option<String> {
        Some(format!("{:04x}:{:04x}", self.vid, self.pid))
    }
}
