use crate::ingest::LogEntry;
use uds_core::LogLevel;

pub struct LogFilter {
    min_level: LogLevel,
}

impl LogFilter {
    pub fn new(min_level: LogLevel) -> Self { Self { min_level } }

    pub fn filter(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().filter(|e| {
            (e.level as u8) >= (self.min_level as u8)
        }).collect()
    }
}
