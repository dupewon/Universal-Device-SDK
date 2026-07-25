use crate::error::ProtocolError;
use crate::MAGIC_BYTES;

const HANDSHAKE_SIZE: usize = 14;

#[derive(Debug, Clone)]
pub struct Handshake {
    pub version_major: u8,
    pub version_minor: u8,
    pub capabilities: u32,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
}

impl Handshake {
    pub fn new(version_major: u8, version_minor: u8) -> Self {
        Self {
            version_major,
            version_minor,
            capabilities: 0,
            device_id: None,
            device_name: None,
        }
    }

    pub fn with_capabilities(mut self, caps: u32) -> Self {
        self.capabilities = caps;
        self
    }

    pub fn with_device_id(mut self, id: &str) -> Self {
        self.device_id = Some(id.to_string());
        self
    }

    pub fn with_device_name(mut self, name: &str) -> Self {
        self.device_name = Some(name.to_string());
        self
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HANDSHAKE_SIZE + 256);
        buf.extend_from_slice(&MAGIC_BYTES);
        buf.push(self.version_major);
        buf.push(self.version_minor);
        buf.extend_from_slice(&self.capabilities.to_le_bytes());

        let id_bytes = self.device_id.as_deref().unwrap_or("").as_bytes();
        buf.push(id_bytes.len() as u8);
        buf.extend_from_slice(id_bytes);

        let name_bytes = self.device_name.as_deref().unwrap_or("").as_bytes();
        buf.push(name_bytes.len() as u8);
        buf.extend_from_slice(name_bytes);

        buf
    }

    pub fn decode(data: &[u8]) -> Result<Self, ProtocolError> {
        if data.len() < HANDSHAKE_SIZE {
            return Err(ProtocolError::FrameTooShort(data.len()));
        }
        if data[..4] != MAGIC_BYTES {
            return Err(ProtocolError::InvalidMagic);
        }

        let version_major = data[4];
        let version_minor = data[5];

        if version_major != crate::PROTOCOL_VERSION_MAJOR {
            return Err(ProtocolError::UnsupportedVersion(version_major));
        }

        let capabilities = u32::from_le_bytes([data[6], data[7], data[8], data[9]]);

        let mut pos = 10;
        let id_len = data[pos] as usize;
        pos += 1;
        let device_id = if id_len > 0 {
            if pos + id_len > data.len() {
                return Err(ProtocolError::FrameTooShort(data.len()));
            }
            Some(
                String::from_utf8(data[pos..pos + id_len].to_vec())
                    .map_err(|_| ProtocolError::InvalidUtf8)?,
            )
        } else {
            None
        };
        pos += id_len;

        let name_len = data[pos] as usize;
        pos += 1;
        let device_name = if name_len > 0 {
            if pos + name_len > data.len() {
                return Err(ProtocolError::FrameTooShort(data.len()));
            }
            Some(
                String::from_utf8(data[pos..pos + name_len].to_vec())
                    .map_err(|_| ProtocolError::InvalidUtf8)?,
            )
        } else {
            None
        };

        Ok(Self {
            version_major,
            version_minor,
            capabilities,
            device_id,
            device_name,
        })
    }

    pub fn negotiate(client: &Handshake, server: &Handshake) -> Result<u8, ProtocolError> {
        let version = client.version_major.min(server.version_major);
        if version < 1 {
            return Err(ProtocolError::HandshakeFailed("no common version".into()));
        }
        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_roundtrip() {
        let hs = Handshake::new(1, 0)
            .with_capabilities(0x0F)
            .with_device_id("esp32-001")
            .with_device_name("DevKit");
        let encoded = hs.encode();
        let decoded = Handshake::decode(&encoded).unwrap();
        assert_eq!(decoded.version_major, 1);
        assert_eq!(decoded.capabilities, 0x0F);
        assert_eq!(decoded.device_id.as_deref(), Some("esp32-001"));
        assert_eq!(decoded.device_name.as_deref(), Some("DevKit"));
    }

    #[test]
    fn test_negotiate_compatible() {
        let client = Handshake::new(1, 0);
        let server = Handshake::new(1, 0);
        assert_eq!(Handshake::negotiate(&client, &server).unwrap(), 1);
    }

    #[test]
    fn test_negotiate_incompatible() {
        let client = Handshake::new(0, 0);
        let server = Handshake::new(1, 0);
        assert!(Handshake::negotiate(&client, &server).is_err());
    }
}
