use uds_rpc::server::RpcServerImpl;
use uds_rpc::message::RpcMessage;
use std::sync::Arc;

pub fn run_rpc(method: &str, params: Option<&str>, _device_id: Option<&str>) -> anyhow::Result<()> {
    let server = RpcServerImpl::new();

    let echo_handler: Arc<dyn Fn(&[u8]) -> Result<Vec<u8>, uds_rpc::error::RpcError> + Send + Sync> =
        Arc::new(|p| Ok(p.to_vec()));
    server.register_method("echo", echo_handler);

    let params_bytes = params
        .map(|p| p.as_bytes().to_vec())
        .unwrap_or_default();

    let request = RpcMessage::request(1, method, &params_bytes, false);
    println!("Calling RPC: {}() with {} bytes", method, params_bytes.len());

    match server.handle_message(&request) {
        Ok(response) => {
            if response.status == Some(0) {
                let payload = response.payload;
                if payload.is_empty() {
                    println!("OK (no data returned)");
                } else {
                    match std::str::from_utf8(&payload) {
                        Ok(s) => println!("Result: {}", s),
                        Err(_) => println!("Result: {} bytes (binary)", payload.len()),
                    }
                }
            } else {
                println!("Error: {}", response.error_msg.unwrap_or_else(|| "unknown".into()));
            }
        }
        Err(e) => {
            anyhow::bail!("RPC call failed: {}", e);
        }
    }

    Ok(())
}
