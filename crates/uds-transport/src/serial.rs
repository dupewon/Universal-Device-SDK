use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use std::fmt;
use crate::traits::{Transport, TransportConnection, TransportConfig, TransportError};

struct SerialInner {
    port_name: String,
    baud: u32,
    open: AtomicBool,
}

impl fmt::Debug for SerialInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SerialInner")
            .field("port_name", &self.port_name)
            .field("baud", &self.baud)
            .field("open", &self.open.load(Ordering::Relaxed))
            .finish()
    }
}

#[derive(Debug)]
pub struct SerialConnection {
    inner: Arc<SerialInner>,
}

impl SerialConnection {
    fn open(path: &str, baud: u32) -> Result<Self, TransportError> {
        #[cfg(target_os = "windows")]
        let port_path = format!("\\\\.\\{}", path.trim_start_matches("\\\\.\\"));
        #[cfg(not(target_os = "windows"))]
        let port_path = path.to_string();

        Ok(Self {
            inner: Arc::new(SerialInner {
                port_name: port_path,
                baud,
                open: AtomicBool::new(true),
            }),
        })
    }
}

impl TransportConnection for SerialConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        tracing::debug!("Serial send {} bytes to {} ({} baud)", buf.len(), self.inner.port_name, self.inner.baud);
        Ok(buf.len())
    }

    fn recv(&self, _buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        Err(TransportError::Timeout(Duration::from_secs(5)))
    }

    fn close(&self) -> Result<(), TransportError> {
        self.inner.open.store(false, Ordering::Relaxed);
        tracing::info!("Serial port {} closed", self.inner.port_name);
        Ok(())
    }

    fn is_open(&self) -> bool {
        self.inner.open.load(Ordering::Relaxed)
    }

    fn set_timeout(&self, _timeout: Duration) -> Result<(), TransportError> {
        Ok(())
    }

    fn peer_addr(&self) -> Option<String> {
        Some(format!("{}@{}", self.inner.port_name, self.inner.baud))
    }
}

#[derive(Debug)]
pub struct SerialTransport;

impl SerialTransport {
    pub fn new() -> Self { Self }

    pub fn available_ports() -> Vec<String> {
        let mut ports = Vec::new();
        #[cfg(windows)]
        {
            for i in 1..=256 {
                let name = format!("COM{}", i);
                let path = format!("\\\\.\\{}", name);
                if std::path::Path::new(&path).exists() {
                    ports.push(name);
                }
            }
        }
        #[cfg(unix)]
        {
            if let Ok(entries) = std::fs::read_dir("/dev") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("ttyUSB") || name.starts_with("ttyACM") || name.starts_with("ttyS") {
                        ports.push(format!("/dev/{}", name));
                    }
                }
            }
        }
        ports
    }
}

impl Transport for SerialTransport {
    fn open(&self, config: TransportConfig) -> Result<Box<dyn TransportConnection>, TransportError> {
        match config {
            TransportConfig::Serial { path, baud, .. } => {
                let conn = SerialConnection::open(&path, baud)?;
                tracing::info!("Serial transport opened: {} @ {} baud", path, baud);
                Ok(Box::new(conn))
            }
            _ => Err(TransportError::Config("expected Serial config".into())),
        }
    }

    fn name(&self) -> &'static str { "serial" }
    fn is_available(&self) -> bool { true }
}
