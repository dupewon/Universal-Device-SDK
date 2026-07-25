use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

pub struct PairingManager {
    pending: Mutex<HashMap<String, String>>,  // device_id -> pin
    paired: Mutex<HashMap<String, Vec<u8>>>,   // device_id -> shared_key
}

impl PairingManager {
    pub fn new() -> Self {
        Self {
            pending: Mutex::new(HashMap::new()),
            paired: Mutex::new(HashMap::new()),
        }
    }

    pub fn generate_pin(&self, device_id: &str) -> String {
        let pin: String = (0..6)
            .map(|_| rand::thread_rng().gen_range(0..10).to_string())
            .collect();
        let mut pending = self.pending.lock().unwrap();
        pending.insert(device_id.to_string(), pin.clone());
        pin
    }

    pub fn verify_pin(&self, device_id: &str, pin: &str) -> bool {
        let mut pending = self.pending.lock().unwrap();
        if let Some(expected) = pending.get(device_id) {
            if expected == pin {
                pending.remove(device_id);
                let mut paired = self.paired.lock().unwrap();
                let key: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
                paired.insert(device_id.to_string(), key);
                return true;
            }
        }
        false
    }

    pub fn is_paired(&self, device_id: &str) -> bool {
        let paired = self.paired.lock().unwrap();
        paired.contains_key(device_id)
    }

    pub fn get_shared_key(&self, device_id: &str) -> Option<Vec<u8>> {
        let paired = self.paired.lock().unwrap();
        paired.get(device_id).cloned()
    }

    pub fn unpair(&self, device_id: &str) {
        let mut paired = self.paired.lock().unwrap();
        paired.remove(device_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairing_flow() {
        let mgr = PairingManager::new();
        let pin = mgr.generate_pin("esp32-001");
        assert_eq!(pin.len(), 6);
        assert!(!mgr.is_paired("esp32-001"));
        assert!(mgr.verify_pin("esp32-001", &pin));
        assert!(mgr.is_paired("esp32-001"));
        assert!(mgr.get_shared_key("esp32-001").is_some());
    }
}
