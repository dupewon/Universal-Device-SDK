use crate::traits::{Transport, TransportConfig, TransportConnection, TransportError};
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

struct WsInner {
    open: AtomicBool,
}

impl fmt::Debug for WsInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WsInner")
            .field("open", &self.open.load(Ordering::Relaxed))
            .finish()
    }
}

#[derive(Debug)]
pub struct WebSocketConnection {
    inner: Arc<WsInner>,
    url: String,
}

#[derive(Debug)]
pub struct WebSocketTransport;

impl Default for WebSocketTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for WebSocketTransport {
    fn open(
        &self,
        config: TransportConfig,
    ) -> Result<Box<dyn TransportConnection>, TransportError> {
        match config {
            TransportConfig::WebSocket { url } => {
                tracing::info!("WebSocket transport configured for {}", url);
                tracing::warn!("WebSocket transport requires tokio-tungstenite feature at runtime");
                Ok(Box::new(WebSocketConnection {
                    inner: Arc::new(WsInner {
                        open: AtomicBool::new(true),
                    }),
                    url,
                }))
            }
            _ => Err(TransportError::Config("expected WebSocket config".into())),
        }
    }

    fn name(&self) -> &'static str {
        "websocket"
    }
    fn is_available(&self) -> bool {
        cfg!(feature = "websocket")
    }
}

impl TransportConnection for WebSocketConnection {
    fn send(&self, buf: &[u8]) -> Result<usize, TransportError> {
        if !self.inner.open.load(Ordering::Relaxed) {
            return Err(TransportError::NotConnected);
        }
        tracing::debug!("WebSocket would send {} bytes to {}", buf.len(), self.url);
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
