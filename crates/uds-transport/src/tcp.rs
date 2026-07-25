use crate::traits::{Transport, TransportConfig, TransportConnection, TransportError};
use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream as StdTcpStream;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

struct TcpInner {
    stream: Arc<Mutex<Option<StdTcpStream>>>,
    open: AtomicBool,
    timeout: Mutex<Duration>,
}

impl fmt::Debug for TcpInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpInner")
            .field("open", &self.open.load(Ordering::Relaxed))
            .finish()
    }
}

#[derive(Debug)]
pub struct TcpConnection {
    inner: Arc<TcpInner>,
}

impl TcpConnection {
    fn connect(host: &str, port: u16, timeout: Duration) -> Result<Self, TransportError> {
        let addr = format!("{}:{}", host, port);
        let stream = StdTcpStream::connect_timeout(
            &addr
                .parse::<std::net::SocketAddr>()
                .map_err(|e: std::net::AddrParseError| TransportError::Config(e.to_string()))?,
            timeout,
        )
        .map_err(|e| TransportError::ConnectionRefused(e.to_string()))?;
        stream.set_read_timeout(Some(timeout)).ok();
        stream.set_write_timeout(Some(timeout)).ok();
        let inner = Arc::new(TcpInner {
            stream: Arc::new(Mutex::new(Some(stream))),
            open: AtomicBool::new(true),
            timeout: Mutex::new(timeout),
        });
        Ok(Self { inner })
    }
}

impl TransportConnection for TcpConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        let mut guard = self.inner.stream.lock().unwrap();
        let stream = guard.as_mut().ok_or(TransportError::NotConnected)?;
        stream
            .write_all(buf)
            .map_err(|e| TransportError::Io(e.to_string()))?;
        stream.flush().ok();
        Ok(buf.len())
    }

    fn recv(&self, buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        let mut guard = self.inner.stream.lock().unwrap();
        let stream = guard.as_mut().ok_or(TransportError::NotConnected)?;
        let n = stream.read(buf).map_err(|e| {
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut
            {
                TransportError::Timeout(*self.inner.timeout.lock().unwrap())
            } else {
                TransportError::Io(e.to_string())
            }
        })?;
        if n == 0 {
            return Err(TransportError::ConnectionRefused(
                "connection closed".into(),
            ));
        }
        Ok(n)
    }

    fn close(&self) -> Result<(), TransportError> {
        self.inner.open.store(false, Ordering::Relaxed);
        let mut guard = self.inner.stream.lock().unwrap();
        if let Some(stream) = guard.take() {
            drop(stream);
        }
        Ok(())
    }

    fn is_open(&self) -> bool {
        self.inner.open.load(Ordering::Relaxed)
    }

    fn set_timeout(&self, timeout: Duration) -> Result<(), TransportError> {
        let mut t = self.inner.timeout.lock().unwrap();
        *t = timeout;
        let guard = self.inner.stream.lock().unwrap();
        if let Some(ref stream) = *guard {
            stream.set_read_timeout(Some(timeout)).ok();
            stream.set_write_timeout(Some(timeout)).ok();
        }
        Ok(())
    }

    fn peer_addr(&self) -> Option<String> {
        let guard = self.inner.stream.lock().unwrap();
        guard
            .as_ref()
            .and_then(|s| s.peer_addr().ok().map(|a| a.to_string()))
    }
}

#[derive(Debug)]
pub struct TcpTransport;

impl TcpTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for TcpTransport {
    fn open(
        &self,
        config: TransportConfig,
    ) -> Result<Box<dyn TransportConnection>, TransportError> {
        match config {
            TransportConfig::Tcp { host, port } => {
                let conn = TcpConnection::connect(&host, port, Duration::from_secs(10))?;
                Ok(Box::new(conn))
            }
            _ => Err(TransportError::Config("expected TCP config".into())),
        }
    }

    fn name(&self) -> &'static str {
        "tcp"
    }
    fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tcp_connection_refused() {
        let transport = TcpTransport::new();
        let result = transport.open(TransportConfig::Tcp {
            host: "127.0.0.1".into(),
            port: 1,
        });
        assert!(result.is_err());
    }
}
