use crate::ingest::LogEntry;
use uds_core::LogLevel;

pub struct LogQuery {
    pub level: Option<LogLevel>,
    pub pattern: Option<String>,
    pub limit: Option<usize>,
}

impl LogQuery {
    pub fn new() -> Self {
        Self {
            level: None,
            pattern: None,
            limit: None,
        }
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn with_pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn execute(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        let filtered = entries.into_iter().filter(|e| {
            let level_ok = self.level.map(|l| e.level == l).unwrap_or(true);
            let pattern_ok = self
                .pattern
                .as_ref()
                .map(|p| e.message.contains(p))
                .unwrap_or(true);
            level_ok && pattern_ok
        });
        match self.limit {
            Some(n) => filtered.take(n).collect(),
            None => filtered.collect(),
        }
    }
}
