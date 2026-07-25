use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MetricSnapshot {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct TelemetryAggregator {
    metrics: Arc<Mutex<HashMap<String, f64>>>,
    history: Arc<Mutex<Vec<MetricSnapshot>>>,
}

impl TelemetryAggregator {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            history: Arc::new(Mutex::new(Vec::with_capacity(10000))),
        }
    }

    pub fn record(&self, name: &str, value: f64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(name.to_string(), value);

        let mut history = self.history.lock().unwrap();
        history.push(MetricSnapshot {
            name: name.to_string(),
            value,
            timestamp: now,
        });
        if history.len() > 10000 {
            history.remove(0);
        }
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        let metrics = self.metrics.lock().unwrap();
        metrics.get(name).copied()
    }

    pub fn snapshot(&self) -> Vec<MetricSnapshot> {
        let metrics = self.metrics.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        metrics.iter().map(|(k, v)| MetricSnapshot {
            name: k.clone(),
            value: *v,
            timestamp: now,
        }).collect()
    }

    pub fn history(&self, name: &str) -> Vec<MetricSnapshot> {
        let history = self.history.lock().unwrap();
        history.iter().filter(|m| m.name == name).cloned().collect()
    }

    pub fn clear(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_retrieve() {
        let telemetry = TelemetryAggregator::new();
        telemetry.record("temperature", 24.5);
        telemetry.record("humidity", 58.2);

        assert_eq!(telemetry.get("temperature"), Some(24.5));
        assert_eq!(telemetry.get("humidity"), Some(58.2));
        assert_eq!(telemetry.get("nonexistent"), None);
    }
}
