pub mod checksum;
pub mod error;
pub mod frame;
pub mod handshake;
pub mod message;

pub use checksum::crc16_ccitt;
pub use error::ProtocolError;
pub use frame::Frame;
pub use handshake::Handshake;
pub use message::{Message, MessageType};

pub const PROTOCOL_VERSION_MAJOR: u8 = 1;
pub const PROTOCOL_VERSION_MINOR: u8 = 0;
pub const MAGIC_BYTES: [u8; 4] = [0x55, 0x44, 0x53, 0x21];
pub const MAX_FRAME_PAYLOAD: u16 = 65535;
