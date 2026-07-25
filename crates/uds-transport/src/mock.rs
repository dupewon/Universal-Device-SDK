use crate::traits::{Transport, TransportConfig, TransportConnection, TransportError};
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

#[derive(Clone)]
struct MockBuffer {
    data: Vec<u8>,
    pos: usize,
}

#[derive(Clone)]
pub struct MockInner {
    write_buf: Arc<Mutex<MockBuffer>>,
    read_buf: Arc<Mutex<MockBuffer>>,
    open: Arc<AtomicBool>,
    timeout: Arc<Mutex<Duration>>,
    latency_ms: u64,
    packet_loss: f64,
}

impl fmt::Debug for MockInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockInner")
            .field("open", &self.open.load(Ordering::Relaxed))
            .field("latency_ms", &self.latency_ms)
            .field("packet_loss", &self.packet_loss)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct MockConnection {
    inner: MockInner,
}

impl Default for MockConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl MockConnection {
    pub fn new() -> Self {
        Self {
            inner: MockInner {
                write_buf: Arc::new(Mutex::new(MockBuffer {
                    data: Vec::new(),
                    pos: 0,
                })),
                read_buf: Arc::new(Mutex::new(MockBuffer {
                    data: Vec::new(),
                    pos: 0,
                })),
                open: Arc::new(AtomicBool::new(true)),
                timeout: Arc::new(Mutex::new(Duration::from_secs(5))),
                latency_ms: 0,
                packet_loss: 0.0,
            },
        }
    }

    pub fn with_latency(mut self, ms: u64) -> Self {
        self.inner.latency_ms = ms;
        self
    }

    pub fn with_packet_loss(mut self, loss: f64) -> Self {
        self.inner.packet_loss = loss;
        self
    }

    pub fn feed_read_data(&self, data: &[u8]) {
        let mut buf = self.inner.read_buf.lock().unwrap();
        buf.data.extend_from_slice(data);
    }

    pub fn drain_written(&self) -> Vec<u8> {
        let mut buf = self.inner.write_buf.lock().unwrap();
        let data = buf.data[buf.pos..].to_vec();
        buf.pos = buf.data.len();
        data
    }
}

impl TransportConnection for MockConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        if self.inner.packet_loss > 0.0 {
            use std::time::{SystemTime, UNIX_EPOCH};
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as f64
                / 1_000_000_000.0;
            if seed < self.inner.packet_loss {
                return Ok(buf.len());
            }
        }
        if self.inner.latency_ms > 0 {
            std::thread::sleep(Duration::from_millis(self.inner.latency_ms));
        }
        let mut wbuf = self.inner.write_buf.lock().unwrap();
        wbuf.data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn recv(&self, buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        let mut rbuf = self.inner.read_buf.lock().unwrap();
        if rbuf.pos >= rbuf.data.len() {
            return Err(TransportError::Timeout(*self.inner.timeout.lock().unwrap()));
        }
        let available = rbuf.data.len() - rbuf.pos;
        let to_copy = available.min(buf.len());
        buf[..to_copy].copy_from_slice(&rbuf.data[rbuf.pos..rbuf.pos + to_copy]);
        rbuf.pos += to_copy;
        Ok(to_copy)
    }

    fn close(&self) -> Result<(), TransportError> {
        self.inner.open.store(false, Ordering::Relaxed);
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

    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct MockTransport;

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for MockTransport {
    fn open(
        &self,
        config: TransportConfig,
    ) -> Result<Box<dyn TransportConnection>, TransportError> {
        let mut conn = MockConnection::new();
        if let TransportConfig::Mock {
            latency_ms,
            packet_loss,
        } = config
        {
            conn.inner.latency_ms = latency_ms;
            conn.inner.packet_loss = packet_loss;
        }
        Ok(Box::new(conn))
    }

    fn name(&self) -> &'static str {
        "mock"
    }
    fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_send_recv() {
        let transport = MockTransport::new();
        let conn = transport
            .open(TransportConfig::Mock {
                latency_ms: 0,
                packet_loss: 0.0,
            })
            .unwrap();
        conn.send(b"hello").unwrap();
        let mc = conn.as_any().downcast_ref::<MockConnection>().unwrap();
        let written = mc.drain_written();
        assert_eq!(written, b"hello");
    }

    #[test]
    fn test_mock_close() {
        let transport = MockTransport::new();
        let conn = transport
            .open(TransportConfig::Mock {
                latency_ms: 0,
                packet_loss: 0.0,
            })
            .unwrap();
        assert!(conn.is_open());
        conn.close().unwrap();
        assert!(!conn.is_open());
        assert!(conn.send(b"x").is_err());
    }
}
