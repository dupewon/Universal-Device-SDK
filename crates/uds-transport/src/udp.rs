use crate::traits::{Transport, TransportConfig, TransportConnection, TransportError};
use std::fmt;
use std::net::UdpSocket as StdUdpSocket;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

const MULTICAST_ADDR: &str = "239.255.0.123";
const MULTICAST_PORT: u16 = 4567;

struct UdpInner {
    socket: Arc<Mutex<Option<StdUdpSocket>>>,
    target: String,
    open: AtomicBool,
    timeout: Mutex<Duration>,
}

impl fmt::Debug for UdpInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UdpInner")
            .field("target", &self.target)
            .field("open", &self.open.load(Ordering::Relaxed))
            .finish()
    }
}

#[derive(Debug)]
pub struct UdpConnection {
    inner: Arc<UdpInner>,
}

impl UdpConnection {
    fn bind(host: &str, port: u16, multicast: Option<&str>) -> Result<Self, TransportError> {
        let bind_addr = format!("0.0.0.0:{}", port);
        let socket =
            StdUdpSocket::bind(&bind_addr).map_err(|e| TransportError::Io(e.to_string()))?;
        socket.set_read_timeout(Some(Duration::from_secs(5))).ok();

        if let Some(mc_addr) = multicast {
            socket
                .join_multicast_v4(
                    &mc_addr.parse::<std::net::Ipv4Addr>().map_err(
                        |e: std::net::AddrParseError| TransportError::Config(e.to_string()),
                    )?,
                    &"0.0.0.0".parse::<std::net::Ipv4Addr>().map_err(
                        |e: std::net::AddrParseError| TransportError::Config(e.to_string()),
                    )?,
                )
                .ok();
        }

        let target = format!("{}:{}", host, port);
        Ok(Self {
            inner: Arc::new(UdpInner {
                socket: Arc::new(Mutex::new(Some(socket))),
                target,
                open: AtomicBool::new(true),
                timeout: Mutex::new(Duration::from_secs(5)),
            }),
        })
    }
}

impl TransportConnection for UdpConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        let guard = self.inner.socket.lock().unwrap();
        let socket = guard.as_ref().ok_or(TransportError::NotConnected)?;
        socket
            .send_to(buf, &self.inner.target)
            .map_err(|e| TransportError::Io(e.to_string()))
    }

    fn recv(&self, buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        let guard = self.inner.socket.lock().unwrap();
        let socket = guard.as_ref().ok_or(TransportError::NotConnected)?;
        let (n, _src) = socket.recv_from(buf).map_err(|e| {
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut
            {
                TransportError::Timeout(*self.inner.timeout.lock().unwrap())
            } else {
                TransportError::Io(e.to_string())
            }
        })?;
        Ok(n)
    }

    fn close(&self) -> Result<(), TransportError> {
        self.inner.open.store(false, Ordering::Relaxed);
        let mut guard = self.inner.socket.lock().unwrap();
        *guard = None;
        Ok(())
    }

    fn is_open(&self) -> bool {
        self.inner.open.load(Ordering::Relaxed)
    }

    fn set_timeout(&self, timeout: Duration) -> Result<(), TransportError> {
        let mut t = self.inner.timeout.lock().unwrap();
        *t = timeout;
        Ok(())
    }

    fn peer_addr(&self) -> Option<String> {
        Some(self.inner.target.clone())
    }
}

#[derive(Debug)]
pub struct UdpTransport;

impl Default for UdpTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl UdpTransport {
    pub fn new() -> Self {
        Self
    }

    pub fn multicast_discovery(timeout: Duration) -> Result<Vec<String>, TransportError> {
        let socket =
            StdUdpSocket::bind("0.0.0.0:0").map_err(|e| TransportError::Io(e.to_string()))?;
        socket.set_broadcast(true).ok();
        socket.set_read_timeout(Some(timeout)).ok();

        let discovery_msg = b"UDS_DISCOVER";
        socket
            .send_to(
                discovery_msg,
                format!("{}:{}", MULTICAST_ADDR, MULTICAST_PORT),
            )
            .ok();

        let mut buf = vec![0u8; 1024];
        let mut devices = Vec::new();
        while let Ok((n, src)) = socket.recv_from(&mut buf) {
            let response = String::from_utf8_lossy(&buf[..n]);
            devices.push(format!("{}: {}", src, response));
        }
        Ok(devices)
    }
}

impl Transport for UdpTransport {
    fn open(
        &self,
        config: TransportConfig,
    ) -> Result<Box<dyn TransportConnection>, TransportError> {
        match config {
            TransportConfig::Udp {
                host,
                port,
                multicast,
            } => {
                let conn = UdpConnection::bind(&host, port, multicast.as_deref())?;
                Ok(Box::new(conn))
            }
            _ => Err(TransportError::Config("expected UDP config".into())),
        }
    }

    fn name(&self) -> &'static str {
        "udp"
    }
    fn is_available(&self) -> bool {
        true
    }
}
