use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LogLine {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
}

pub struct MonitorCapture {
    buffer: Arc<Mutex<Vec<LogLine>>>,
    running: Arc<Mutex<bool>>,
}

impl Default for MonitorCapture {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorCapture {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::with_capacity(1000))),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        *running = true;
        tracing::info!("Monitor capture started");
    }

    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
        tracing::info!("Monitor capture stopped");
    }

    pub fn feed(&self, level: &str, message: &str) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut buf = self.buffer.lock().unwrap();
        buf.push(LogLine {
            timestamp: ts,
            level: level.to_string(),
            message: message.to_string(),
        });
        if buf.len() > 1000 {
            buf.remove(0);
        }
    }

    pub fn drain(&self) -> Vec<LogLine> {
        let mut buf = self.buffer.lock().unwrap();
        buf.drain(..).collect()
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_feed_drain() {
        let cap = MonitorCapture::new();
        cap.feed("info", "test message");
        let lines = cap.drain();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].level, "info");
        assert_eq!(lines[0].message, "test message");
    }

    #[test]
    fn test_capture_buffer_limited() {
        let cap = MonitorCapture::new();
        for i in 0..2000 {
            cap.feed("debug", &format!("message {}", i));
        }
        let lines = cap.drain();
        assert!(lines.len() <= 1000);
    }
}
