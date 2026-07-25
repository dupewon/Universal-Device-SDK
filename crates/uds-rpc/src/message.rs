use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct RpcMessage {
    pub seq: u16,
    pub msg_type: u8,
    pub method: Option<String>,
    pub payload: Bytes,
    pub stream_id: Option<u32>,
    pub status: Option<u32>,
    pub error_msg: Option<String>,
}

impl RpcMessage {
    pub fn request(seq: u16, method: &str, params: &[u8], streaming: bool) -> Self {
        Self {
            seq,
            msg_type: if streaming { 3 } else { 0 },
            method: Some(method.to_string()),
            payload: Bytes::copy_from_slice(params),
            stream_id: None,
            status: None,
            error_msg: None,
        }
    }

    pub fn response(seq: u16, payload: &[u8], status: u32, error: Option<String>) -> Self {
        Self {
            seq,
            msg_type: 1,
            method: None,
            payload: Bytes::copy_from_slice(payload),
            stream_id: None,
            status: Some(status),
            error_msg: error,
        }
    }

    pub fn notification(seq: u16, method: &str, data: &[u8]) -> Self {
        Self {
            seq,
            msg_type: 2,
            method: Some(method.to_string()),
            payload: Bytes::copy_from_slice(data),
            stream_id: None,
            status: None,
            error_msg: None,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put_u8(self.msg_type);
        buf.put_u16_le(self.seq);

        match &self.method {
            Some(m) => {
                buf.put_u8(1);
                let mb = m.as_bytes();
                buf.put_u16_le(mb.len() as u16);
                buf.put_slice(mb);
            }
            None => buf.put_u8(0),
        }

        match self.stream_id {
            Some(id) => {
                buf.put_u8(1);
                buf.put_u32_le(id);
            }
            None => buf.put_u8(0),
        }

        match self.status {
            Some(s) => {
                buf.put_u8(1);
                buf.put_u32_le(s);
            }
            None => buf.put_u8(0),
        }

        match &self.error_msg {
            Some(e) => {
                buf.put_u8(1);
                let eb = e.as_bytes();
                buf.put_u16_le(eb.len() as u16);
                buf.put_slice(eb);
            }
            None => buf.put_u8(0),
        }

        buf.put_u32_le(self.payload.len() as u32);
        buf.put_slice(&self.payload);
        buf.freeze().to_vec()
    }

    pub fn decode(data: &[u8]) -> Result<Self, crate::error::RpcError> {
        let mut pos = 0;
        if data.len() < 3 {
            return Err(crate::error::RpcError::Transport(
                "message too short".into(),
            ));
        }
        let msg_type = data[pos];
        pos += 1;
        let seq = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let method = if pos < data.len() && data[pos] == 1 {
            pos += 1;
            if pos + 2 > data.len() {
                return Err(crate::error::RpcError::Transport("truncated method".into()));
            }
            let mlen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + mlen > data.len() {
                return Err(crate::error::RpcError::Transport(
                    "truncated method name".into(),
                ));
            }
            let m = String::from_utf8(data[pos..pos + mlen].to_vec())
                .map_err(|_| crate::error::RpcError::Protocol("invalid UTF-8".into()))?;
            pos += mlen;
            Some(m)
        } else {
            pos += 1;
            None
        };

        let stream_id = if pos < data.len() && data[pos] == 1 {
            pos += 1;
            if pos + 4 > data.len() {
                return Err(crate::error::RpcError::Transport(
                    "truncated stream id".into(),
                ));
            }
            let id = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            Some(id)
        } else {
            pos += 1;
            None
        };

        let status = if pos < data.len() && data[pos] == 1 {
            pos += 1;
            if pos + 4 > data.len() {
                return Err(crate::error::RpcError::Transport("truncated status".into()));
            }
            let s = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            Some(s)
        } else {
            pos += 1;
            None
        };

        let error_msg = if pos < data.len() && data[pos] == 1 {
            pos += 1;
            if pos + 2 > data.len() {
                return Err(crate::error::RpcError::Transport("truncated error".into()));
            }
            let elen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + elen > data.len() {
                return Err(crate::error::RpcError::Transport(
                    "truncated error msg".into(),
                ));
            }
            let e = String::from_utf8(data[pos..pos + elen].to_vec())
                .map_err(|_| crate::error::RpcError::Protocol("invalid UTF-8".into()))?;
            pos += elen;
            Some(e)
        } else {
            pos += 1;
            None
        };

        if pos + 4 > data.len() {
            return Err(crate::error::RpcError::Transport(
                "truncated payload length".into(),
            ));
        }
        let plen =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        if pos + plen > data.len() {
            return Err(crate::error::RpcError::Transport(
                "truncated payload".into(),
            ));
        }

        Ok(Self {
            seq,
            msg_type,
            method,
            stream_id,
            status,
            error_msg,
            payload: Bytes::copy_from_slice(&data[pos..pos + plen]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_roundtrip() {
        let msg = RpcMessage::request(42, "GetStatus", &[1, 2, 3], false);
        let encoded = msg.encode();
        let decoded = RpcMessage::decode(&encoded).unwrap();
        assert_eq!(decoded.seq, 42);
        assert_eq!(decoded.method.as_deref(), Some("GetStatus"));
        assert_eq!(decoded.payload.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn test_response_with_error() {
        let msg = RpcMessage::response(7, &[], 1, Some("not found".into()));
        let encoded = msg.encode();
        let decoded = RpcMessage::decode(&encoded).unwrap();
        assert_eq!(decoded.status, Some(1));
        assert_eq!(decoded.error_msg.as_deref(), Some("not found"));
    }
}
