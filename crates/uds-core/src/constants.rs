pub const MAX_PAYLOAD_SIZE: usize = 65535;
pub const DEFAULT_PORT: u16 = 4567;
pub const MULTICAST_ADDR: &str = "239.255.0.123";
pub const MAGIC_BYTES: [u8; 4] = [0x55, 0x44, 0x53, 0x21];
pub const UDS_PROTOCOL_VERSION_MAJOR: u8 = 1;
pub const UDS_PROTOCOL_VERSION_MINOR: u8 = 0;
pub const CONFIG_DIR: &str = ".uds";
pub const CONFIG_FILE: &str = "config.toml";
