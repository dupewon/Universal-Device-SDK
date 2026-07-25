use crate::logging::FirmwareLogger;
use crate::ota::OtaClient;
use crate::transport::FirmwareTransport;
use uds_core::types::DeviceInfo;
use uds_protocol::message::{Message, MessageType};

pub struct UdsClient<T: FirmwareTransport> {
    transport: T,
    ota: OtaClient,
    logger: FirmwareLogger,
    device_info: DeviceInfo,
    connected: bool,
    frame_buf: [u8; 4096],
}

impl<T: FirmwareTransport> UdsClient<T> {
    pub fn new(transport: T, device_info: DeviceInfo) -> Self {
        Self {
            transport,
            ota: OtaClient::new(),
            logger: FirmwareLogger::new(),
            device_info,
            connected: false,
            frame_buf: [0u8; 4096],
        }
    }

    pub fn connect(&mut self) -> Result<(), uds_core::error::Error> {
        let mut handshake_buf = [0u8; 256];
        match self.transport.recv(&mut handshake_buf) {
            Ok(n) if n > 0 => {
                if let Ok(msg) = Message::decode(&handshake_buf[..n]) {
                    if msg.msg_type == MessageType::Request
                        && msg.method.as_deref() == Some("handshake")
                    {
                        let ack = Message {
                            msg_type: MessageType::Response,
                            method: Some("handshake".into()),
                            payload: bytes::Bytes::new(),
                            stream_id: None,
                            seq: msg.seq,
                            status: Some(0),
                            error_msg: None,
                        };
                        let out = ack.encode();
                        self.transport.send(&out)?;
                        self.connected = true;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn poll(&mut self) -> Result<Option<Message>, uds_core::error::Error> {
        if !self.connected {
            return Ok(None);
        }
        let n = self.transport.recv(&mut self.frame_buf)?;
        if n == 0 {
            return Ok(None);
        }
        let msg = Message::decode(&self.frame_buf[..n])
            .map_err(|e| uds_core::error::Error::Protocol(e.to_string()))?;

        match msg.msg_type {
            MessageType::Request if msg.method.as_deref() == Some("ota_chunk") => {
                self.ota.receive_chunk(0, &msg.payload);
            }
            MessageType::Request
                if msg.method.as_deref() == Some("ota_commit") && self.ota.verify() =>
            {
                self.ota.apply();
            }
            _ => {}
        }
        Ok(Some(msg))
    }

    pub fn send_log(&mut self, level: u8, message: &str) -> Result<(), uds_core::error::Error> {
        let log_msg = Message {
            msg_type: MessageType::Notification,
            method: Some("log".into()),
            payload: bytes::Bytes::from(message.as_bytes().to_vec()),
            stream_id: None,
            seq: 0,
            status: Some(level as u32),
            error_msg: None,
        };
        let out = log_msg.encode();
        self.transport.send(&out)
    }

    pub fn flush_logs(&mut self) -> Result<(), uds_core::error::Error> {
        self.logger.drain_to(|level, msg_bytes, ts| {
            let log_msg = Message {
                msg_type: MessageType::Notification,
                method: Some("log".into()),
                payload: bytes::Bytes::from(msg_bytes.to_vec()),
                stream_id: None,
                seq: 0,
                status: Some(level as u32),
                error_msg: Some(ts.to_string()),
            };
            let out = log_msg.encode();
            let _ = self.transport.send(&out);
        });
        Ok(())
    }

    pub fn logger(&mut self) -> &mut FirmwareLogger {
        &mut self.logger
    }

    pub fn ota(&mut self) -> &mut OtaClient {
        &mut self.ota
    }

    pub fn device_info(&self) -> &DeviceInfo {
        &self.device_info
    }

    pub fn into_transport(self) -> T {
        self.transport
    }
}
