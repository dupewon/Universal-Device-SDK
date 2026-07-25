use crate::error::RpcError;
use crate::message::RpcMessage;
use std::fmt;
use std::sync::atomic::{AtomicU16, Ordering};

pub trait RpcClient: Send + Sync + fmt::Debug {
    fn call(&self, method: &str, params: &[u8]) -> Result<Vec<u8>, RpcError>;
    fn call_streaming(
        &self,
        method: &str,
        params: &[u8],
    ) -> Result<Box<dyn StreamReceiver>, RpcError>;
    fn notify(&self, method: &str, data: &[u8]) -> Result<(), RpcError>;
}

pub trait StreamReceiver: Send + Sync {
    fn recv(&mut self) -> Option<Vec<u8>>;
    fn close(&mut self);
}

pub struct RpcClientImpl {
    transport: Box<dyn uds_transport::traits::TransportConnection>,
    next_seq: AtomicU16,
}

impl fmt::Debug for RpcClientImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RpcClientImpl").finish()
    }
}

impl RpcClientImpl {
    pub fn new(transport: Box<dyn uds_transport::traits::TransportConnection>) -> Self {
        Self {
            transport,
            next_seq: AtomicU16::new(1),
        }
    }

    fn next_seq(&self) -> u16 {
        self.next_seq.fetch_add(1, Ordering::SeqCst)
    }

    pub fn send_frame(&self, msg: &RpcMessage) -> Result<(), RpcError> {
        let data = msg.encode();
        self.transport
            .send(&data)
            .map_err(|e| RpcError::Transport(e.to_string()))?;
        Ok(())
    }

    pub fn recv_frame(&self) -> Result<RpcMessage, RpcError> {
        let mut header = [0u8; uds_protocol::frame::HEADER_SIZE];
        let n = self
            .transport
            .recv(&mut header)
            .map_err(|e| RpcError::Transport(e.to_string()))?;
        if n < uds_protocol::frame::HEADER_SIZE {
            return Err(RpcError::Transport("incomplete header".into()));
        }

        let frame = uds_protocol::frame::Frame::decode(&header)
            .map_err(|e| RpcError::Protocol(e.to_string()))?;

        if frame.payload.is_empty() {
            return Err(RpcError::Transport("empty payload".into()));
        }

        RpcMessage::decode(&frame.payload).map_err(|e| RpcError::Protocol(e.to_string()))
    }

    fn do_call(
        &self,
        method: &str,
        params: &[u8],
        streaming: bool,
    ) -> Result<RpcMessage, RpcError> {
        let seq = self.next_seq();
        let msg = RpcMessage::request(seq, method, params, streaming);
        self.send_frame(&msg)?;

        let response = self.recv_frame()?;
        if response.status.is_some_and(|s| s != 0) {
            let err_msg = response.error_msg.unwrap_or_else(|| "unknown error".into());
            return Err(RpcError::RemoteError(err_msg));
        }
        Ok(response)
    }
}

impl RpcClient for RpcClientImpl {
    fn call(&self, method: &str, params: &[u8]) -> Result<Vec<u8>, RpcError> {
        let response = self.do_call(method, params, false)?;
        Ok(response.payload.to_vec())
    }

    fn call_streaming(
        &self,
        method: &str,
        params: &[u8],
    ) -> Result<Box<dyn StreamReceiver>, RpcError> {
        let _response = self.do_call(method, params, true)?;
        Err(RpcError::NotSupported(
            "streaming not yet implemented".into(),
        ))
    }

    fn notify(&self, method: &str, data: &[u8]) -> Result<(), RpcError> {
        let seq = self.next_seq();
        let msg = RpcMessage::notification(seq, method, data);
        self.send_frame(&msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uds_transport::mock::MockTransport;
    use uds_transport::traits::{Transport, TransportConfig, TransportConnection};

    #[test]
    fn test_notify_roundtrip() {
        let transport = MockTransport::new();
        let conn = transport
            .open(TransportConfig::Mock {
                latency_ms: 0,
                packet_loss: 0.0,
            })
            .unwrap();
        let client = RpcClientImpl::new(conn);

        client.notify("test", b"hello").unwrap();
        // Should not crash
    }
}
