use crate::traits::Transport;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct TransportRegistry {
    transports: RwLock<HashMap<&'static str, Arc<dyn Transport>>>,
}

impl TransportRegistry {
    pub fn new() -> Self {
        Self {
            transports: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, transport: Arc<dyn Transport>) {
        let mut map = self.transports.write().unwrap();
        map.insert(transport.name(), transport);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Transport>> {
        let map = self.transports.read().unwrap();
        map.get(name).cloned()
    }

    pub fn list(&self) -> Vec<&'static str> {
        let map = self.transports.read().unwrap();
        let mut keys: Vec<_> = map.keys().copied().collect();
        keys.sort();
        keys
    }

    pub fn has(&self, name: &str) -> bool {
        let map = self.transports.read().unwrap();
        map.contains_key(name)
    }

    pub fn from_config(
        config: &crate::traits::TransportConfig,
        registry: &Self,
    ) -> Result<Arc<dyn Transport>, crate::traits::TransportError> {
        let name = match config {
            crate::traits::TransportConfig::Serial { .. } => "serial",
            crate::traits::TransportConfig::Tcp { .. } => "tcp",
            crate::traits::TransportConfig::Udp { .. } => "udp",
            crate::traits::TransportConfig::WebSocket { .. } => "websocket",
            crate::traits::TransportConfig::Ble { .. } => "ble",
            crate::traits::TransportConfig::Usb { .. } => "usb",
            crate::traits::TransportConfig::Mock { .. } => "mock",
        };
        registry.get(name).ok_or_else(|| {
            crate::traits::TransportError::Config(format!("transport '{}' not registered", name))
        })
    }
}

impl Default for TransportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockTransport;

    #[test]
    fn test_register_and_list() {
        let reg = TransportRegistry::new();
        reg.register(Arc::new(MockTransport::new()));
        assert!(reg.has("mock"));
        assert_eq!(reg.list(), vec!["mock"]);
    }
}
