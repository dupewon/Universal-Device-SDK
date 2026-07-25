#[cfg(test)]
mod tests {
    #[test]
    fn test_mock_device_flow() {
        let transport = uds_transport::MockTransport::new();
        let config = uds_transport::traits::TransportConfig::Mock { latency_ms: 0, packet_loss: 0.0 };
        let conn = transport.open(config).unwrap();
        assert!(conn.is_open());
        conn.send(b"test").unwrap();
        conn.close().unwrap();
        assert!(!conn.is_open());
    }
}
