#[cfg(test)]
mod tests {
    use uds_transport::traits::{Transport, TransportConfig, TransportConnection};
    use uds_transport::mock::MockTransport;

    #[test]
    fn test_mock_transport_send_recv() {
        let transport = MockTransport::new();
        let conn = transport.open(TransportConfig::Mock { latency_ms: 0, packet_loss: 0.0 }).unwrap();

        let data = b"hello uds";
        conn.send(data).unwrap();

        // Get the mock connection to read written data
        let mc = conn.as_any()
            .and_then(|a| a.downcast_ref::<uds_transport::mock::MockConnection>())
            .expect("connection should be mock");
        let written = mc.drain_written();
        mc.feed_read_data(&written);

        let mut buf = vec![0u8; 64];
        let n = conn.recv(&mut buf).unwrap();
        assert_eq!(&buf[..n], data);
    }
}
