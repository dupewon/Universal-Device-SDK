use std::sync::{Arc, Mutex};
use uds_core::LogLevel;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub target: String,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct LogIngester {
    buffer: Arc<Mutex<Vec<LogEntry>>>,
}

impl LogIngester {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, entry: LogEntry) {
        let mut buf = self.buffer.lock().unwrap();
        buf.push(entry);
    }

    pub fn drain(&self) -> Vec<LogEntry> {
        let mut buf = self.buffer.lock().unwrap();
        buf.drain(..).collect()
    }

    pub fn query(&self, query: crate::query::LogQuery) -> Vec<LogEntry> {
        let buf = self.buffer.lock().unwrap();
        query.execute(buf.clone())
    }
}
