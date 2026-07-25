use crate::error::RpcError;
use crate::message::RpcMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type RpcHandler = Arc<dyn Fn(&[u8]) -> Result<Vec<u8>, RpcError> + Send + Sync>;

pub trait RpcServer: Send + Sync {
    fn register_method(&self, name: &str, handler: RpcHandler);
    fn handle_message(&self, msg: &RpcMessage) -> Result<RpcMessage, RpcError>;
    fn list_methods(&self) -> Vec<String>;
}

pub struct RpcServerImpl {
    methods: Mutex<HashMap<String, RpcHandler>>,
}

impl Default for RpcServerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl RpcServerImpl {
    pub fn new() -> Self {
        Self {
            methods: Mutex::new(HashMap::new()),
        }
    }
}

impl RpcServer for RpcServerImpl {
    fn register_method(&self, name: &str, handler: RpcHandler) {
        let mut methods = self.methods.lock().unwrap();
        methods.insert(name.to_string(), handler);
    }

    fn handle_message(&self, msg: &RpcMessage) -> Result<RpcMessage, RpcError> {
        let method_name = msg.method.as_deref().unwrap_or("");
        let handler = {
            let methods = self.methods.lock().unwrap();
            methods.get(method_name).cloned()
        };

        match handler {
            Some(h) => {
                let result = h(&msg.payload)?;
                Ok(RpcMessage::response(msg.seq, &result, 0, None))
            }
            None => Ok(RpcMessage::response(
                msg.seq,
                &[],
                1,
                Some(format!("method '{}' not found", method_name)),
            )),
        }
    }

    fn list_methods(&self) -> Vec<String> {
        let methods = self.methods.lock().unwrap();
        let mut names: Vec<String> = methods.keys().cloned().collect();
        names.sort();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_call() {
        let server = RpcServerImpl::new();
        let handler: RpcHandler = Arc::new(|params| Ok(format!("echo: {:?}", params).into_bytes()));
        server.register_method("Echo", handler);

        let methods = server.list_methods();
        assert_eq!(methods, vec!["Echo"]);

        let req = RpcMessage::request(1, "Echo", b"hello", false);
        let resp = server.handle_message(&req).unwrap();
        assert_eq!(resp.status, Some(0));
        assert_eq!(resp.payload.as_ref(), b"echo: [104, 101, 108, 108, 111]");
    }

    #[test]
    fn test_method_not_found() {
        let server = RpcServerImpl::new();
        let req = RpcMessage::request(1, "NonExistent", b"", false);
        let resp = server.handle_message(&req).unwrap();
        assert_eq!(resp.status, Some(1));
        assert!(resp.error_msg.unwrap().contains("NonExistent"));
    }
}
