use uds_rpc::client::RpcClientImpl;
use uds_transport::serial::SerialTransport;
use uds_transport::traits::{Transport, TransportConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let transport = SerialTransport::new();
    let config = TransportConfig::Serial {
        path: "/dev/ttyUSB0".into(),
        baud: 115200,
        parity: None,
        stop_bits: None,
    };
    let conn = transport.open(config)?;

    let client = RpcClientImpl::new(conn);
    let response = client.call("GetStatus", &[])?;
    println!("Device status: {:?}", response);

    Ok(())
}
