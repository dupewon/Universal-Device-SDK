use crate::checksum::crc16_ccitt;
use crate::error::ProtocolError;
use crate::MAGIC_BYTES;
use bytes::{BufMut, Bytes, BytesMut};

pub const HEADER_SIZE: usize = 12;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFlag {
    Request = 0x01,
    Response = 0x02,
    Streaming = 0x04,
    Compressed = 0x08,
    Encrypted = 0x10,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub version: u8,
    pub flags: u8,
    pub sequence: u16,
    pub payload: Bytes,
}

impl Frame {
    pub fn new(
        version: u8,
        flags: u8,
        sequence: u16,
        payload: &[u8],
    ) -> Result<Self, ProtocolError> {
        if payload.len() > crate::MAX_FRAME_PAYLOAD as usize {
            return Err(ProtocolError::PayloadTooLong(payload.len()));
        }
        Ok(Self {
            version,
            flags,
            sequence,
            payload: Bytes::copy_from_slice(payload),
        })
    }

    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(HEADER_SIZE + self.payload.len());
        buf.put_slice(&MAGIC_BYTES);
        buf.put_u8(self.version);
        buf.put_u8(self.flags);
        buf.put_u16_le(self.sequence);
        buf.put_u16_le(self.payload.len() as u16);
        let checksum = if self.flags & FrameFlag::Encrypted as u8 == 0 {
            crc16_ccitt(&self.payload)
        } else {
            0
        };
        buf.put_u16_le(checksum);
        buf.put_slice(&self.payload);
        buf.freeze()
    }

    pub fn decode(data: &[u8]) -> Result<Self, ProtocolError> {
        if data.len() < HEADER_SIZE {
            return Err(ProtocolError::FrameTooShort(data.len()));
        }
        if data[..4] != MAGIC_BYTES {
            return Err(ProtocolError::InvalidMagic);
        }

        let version = data[4];
        let flags = data[5];
        let sequence = u16::from_le_bytes([data[6], data[7]]);
        let payload_len = u16::from_le_bytes([data[8], data[9]]) as usize;
        let stored_checksum = u16::from_le_bytes([data[10], data[11]]);

        if data.len() < HEADER_SIZE + payload_len {
            return Err(ProtocolError::FrameTooShort(data.len()));
        }

        let payload = Bytes::copy_from_slice(&data[HEADER_SIZE..HEADER_SIZE + payload_len]);

        if stored_checksum != 0 && (flags & FrameFlag::Encrypted as u8) == 0 {
            let computed = crc16_ccitt(&payload);
            if computed != stored_checksum {
                return Err(ProtocolError::ChecksumMismatch {
                    expected: stored_checksum,
                    got: computed,
                });
            }
        }

        Ok(Self {
            version,
            flags,
            sequence,
            payload,
        })
    }

    pub fn has_flag(&self, flag: FrameFlag) -> bool {
        self.flags & flag as u8 != 0
    }

    pub fn is_request(&self) -> bool {
        self.has_flag(FrameFlag::Request)
    }
    pub fn is_response(&self) -> bool {
        self.has_flag(FrameFlag::Response)
    }
    pub fn is_streaming(&self) -> bool {
        self.has_flag(FrameFlag::Streaming)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_encode_decode() {
        let payload = b"hello uds protocol";
        let frame = Frame::new(1, FrameFlag::Request as u8, 1, payload).unwrap();
        let encoded = frame.encode();
        let decoded = Frame::decode(&encoded).unwrap();
        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.sequence, 1);
        assert_eq!(decoded.payload.as_ref(), payload);
        assert!(decoded.is_request());
    }

    #[test]
    fn test_frame_checksum_mismatch() {
        let mut buf = vec![0u8; HEADER_SIZE + 4];
        buf[..4].copy_from_slice(&MAGIC_BYTES);
        buf[8] = 4; // payload length
        buf[10] = 0xAB; // non-zero checksum → will trigger validation and fail
        buf[11] = 0xCD;
        let result = Frame::decode(&buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_payload() {
        let frame = Frame::new(1, 0, 0, &[]).unwrap();
        let encoded = frame.encode();
        assert_eq!(encoded.len(), HEADER_SIZE);
        let decoded = Frame::decode(&encoded).unwrap();
        assert!(decoded.payload.is_empty());
    }
}
