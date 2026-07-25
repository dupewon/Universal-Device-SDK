#[cfg(test)]
mod tests {
    use uds_protocol::{Frame, Message, checksum::crc16_ccitt};

    #[test]
    fn test_frame_roundtrip() {
        let payload = b"test payload";
        let frame = Frame::new(1, 0, 0, payload).unwrap();
        let encoded = frame.encode();
        let decoded = Frame::decode(&encoded).unwrap();
        assert_eq!(frame.sequence, decoded.sequence);
        assert_eq!(frame.payload, decoded.payload);
    }

    #[test]
    fn test_crc16() {
        let data = b"hello";
        let crc = crc16_ccitt(data);
        assert_ne!(crc, 0);
    }
}
