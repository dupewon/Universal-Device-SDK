use bytes::{Bytes, BytesMut, BufMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Request = 0,
    Response = 1,
    Notification = 2,
    StreamHeader = 3,
    StreamData = 4,
    StreamEnd = 5,
}

impl MessageType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Request),
            1 => Some(Self::Response),
            2 => Some(Self::Notification),
            3 => Some(Self::StreamHeader),
            4 => Some(Self::StreamData),
            5 => Some(Self::StreamEnd),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub msg_type: MessageType,
    pub method: Option<String>,
    pub payload: Bytes,
    pub stream_id: Option<u32>,
    pub seq: u16,
    pub status: Option<u32>,
    pub error_msg: Option<String>,
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put_u8(self.msg_type as u8);
        buf.put_u16(self.seq);

        match &self.method {
            Some(m) => {
                buf.put_u8(1);
                let mb = m.as_bytes();
                buf.put_u16(mb.len() as u16);
                buf.put_slice(mb);
            }
            None => buf.put_u8(0),
        }

        match self.stream_id {
            Some(id) => {
                buf.put_u8(1);
                buf.put_u32(id);
            }
            None => buf.put_u8(0),
        }

        match self.status {
            Some(s) => {
                buf.put_u8(1);
                buf.put_u32(s);
            }
            None => buf.put_u8(0),
        }

        match &self.error_msg {
            Some(e) => {
                buf.put_u8(1);
                let eb = e.as_bytes();
                buf.put_u16(eb.len() as u16);
                buf.put_slice(eb);
            }
            None => buf.put_u8(0),
        }

        buf.put_u32(self.payload.len() as u32);
        buf.put_slice(&self.payload);
        buf.freeze().to_vec()
    }

    pub fn decode(data: &[u8]) -> Result<Self, crate::error::ProtocolError> {
        let mut pos = 0;
        if data.len() < 5 {
            return Err(crate::error::ProtocolError::FrameTooShort(data.len()));
        }

        let msg_type = MessageType::from_u8(data[pos])
            .ok_or(crate::error::ProtocolError::InvalidMessageType(data[pos]))?;
        pos += 1;

        let seq = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let method = if data[pos] == 1 {
            pos += 1;
            if pos + 2 > data.len() { return Err(crate::error::ProtocolError::FrameTooShort(data.len())); }
            let mlen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + mlen > data.len() { return Err(crate::error::ProtocolError::FrameTooShort(data.len())); }
            let m = String::from_utf8(data[pos..pos + mlen].to_vec())
                .map_err(|_| crate::error::ProtocolError::InvalidUtf8)?;
            pos += mlen;
            Some(m)
        } else {
            pos += 1;
            None
        };

        let stream_id = if data[pos] == 1 {
            pos += 1;
            if pos + 4 > data.len() { return Err(crate::error::ProtocolError::FrameTooShort(data.len())); }
            let id = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            Some(id)
        } else {
            pos += 1;
            None
        };

        let status = if data[pos] == 1 {
            pos += 1;
            if pos + 4 > data.len() { return Err(crate::error::ProtocolError::FrameTooShort(data.len())); }
            let s = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            Some(s)
        } else {
            pos += 1;
            None
        };

        let error_msg = if data[pos] == 1 {
            pos += 1;
            if pos + 2 > data.len() { return Err(crate::error::ProtocolError::FrameTooShort(data.len())); }
            let elen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + elen > data.len() { return Err(crate::error::ProtocolError::FrameTooShort(data.len())); }
            let e = String::from_utf8(data[pos..pos + elen].to_vec())
                .map_err(|_| crate::error::ProtocolError::InvalidUtf8)?;
            pos += elen;
            Some(e)
        } else {
            pos += 1;
            None
        };

        if pos + 4 > data.len() {
            return Err(crate::error::ProtocolError::FrameTooShort(data.len()));
        }
        let plen = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        if pos + plen > data.len() {
            return Err(crate::error::ProtocolError::FrameTooShort(data.len()));
        }

        let payload = Bytes::copy_from_slice(&data[pos..pos + plen]);

        Ok(Self { msg_type, method, payload, stream_id, seq, status, error_msg })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_roundtrip() {
        let msg = Message {
            msg_type: MessageType::Request,
            method: Some("GetStatus".into()),
            payload: Bytes::from(&b"\x01\x02\x03"[..]),
            stream_id: None,
            seq: 42,
            status: None,
            error_msg: None,
        };
        let encoded = msg.encode();
        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(decoded.msg_type, MessageType::Request);
        assert_eq!(decoded.method.as_deref(), Some("GetStatus"));
        assert_eq!(decoded.seq, 42);
        assert_eq!(decoded.payload.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn test_message_response_with_error() {
        let msg = Message {
            msg_type: MessageType::Response,
            method: None,
            payload: Bytes::new(),
            stream_id: None,
            seq: 7,
            status: Some(1),
            error_msg: Some("device not found".into()),
        };
        let encoded = msg.encode();
        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(decoded.status, Some(1));
        assert_eq!(decoded.error_msg.as_deref(), Some("device not found"));
    }
}
