use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum TransportConfig {
    Serial {
        path: String,
        baud: u32,
        parity: Option<String>,
        stop_bits: Option<u8>,
    },
    Tcp {
        host: String,
        port: u16,
    },
    Udp {
        host: String,
        port: u16,
        multicast: Option<String>,
    },
    WebSocket {
        url: String,
    },
    Ble {
        mac: String,
        service_uuid: Option<String>,
    },
    Usb {
        vid: u16,
        pid: u16,
        interface: Option<u8>,
    },
    Mock {
        latency_ms: u64,
        packet_loss: f64,
    },
}

impl fmt::Display for TransportConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serial { path, baud, .. } => write!(f, "serial:{}@{}", path, baud),
            Self::Tcp { host, port } => write!(f, "tcp:{}:{}", host, port),
            Self::Udp { host, port, .. } => write!(f, "udp:{}:{}", host, port),
            Self::WebSocket { url } => write!(f, "ws:{}", url),
            Self::Ble { mac, .. } => write!(f, "ble:{}", mac),
            Self::Usb { vid, pid, .. } => write!(f, "usb:{:04x}:{:04x}", vid, pid),
            Self::Mock { .. } => write!(f, "mock"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("not connected")]
    NotConnected,
    #[error("timeout after {0:?}")]
    Timeout(Duration),
    #[error("configuration error: {0}")]
    Config(String),
    #[error("transport unavailable: {0}")]
    Unavailable(String),
    #[error("connection refused: {0}")]
    ConnectionRefused(String),
    #[error("protocol violation: {0}")]
    ProtocolViolation(String),
}

impl From<std::io::Error> for TransportError {
    fn from(e: std::io::Error) -> Self {
        TransportError::Io(e.to_string())
    }
}

pub trait TransportConnection: Send + Sync + fmt::Debug {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError>;
    fn recv(&self, buf: &mut [u8]) -> Result<usize, TransportError>;
    fn close(&self) -> Result<(), TransportError>;
    fn is_open(&self) -> bool;
    fn set_timeout(&self, timeout: Duration) -> Result<(), TransportError>;
    fn peer_addr(&self) -> Option<String> {
        None
    }
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        None
    }
}

pub trait Transport: Send + Sync + fmt::Debug {
    fn open(&self, config: TransportConfig)
        -> Result<Box<dyn TransportConnection>, TransportError>;
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;
}
