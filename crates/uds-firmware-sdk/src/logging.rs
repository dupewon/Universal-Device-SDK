const LOG_BUFFER_CAPACITY: usize = 32;

struct LogEntry {
    level: u8,
    message: [u8; 128],
    len: u8,
    timestamp: u64,
}

pub struct FirmwareLogger {
    buffer: [LogEntry; LOG_BUFFER_CAPACITY],
    head: usize,
    count: usize,
}

impl FirmwareLogger {
    pub fn new() -> Self {
        Self {
            buffer: [LogEntry { level: 0, message: [0u8; 128], len: 0, timestamp: 0 }; LOG_BUFFER_CAPACITY],
            head: 0,
            count: 0,
        }
    }

    pub fn log(&mut self, level: u8, message: &str, timestamp: u64) {
        let entry = &mut self.buffer[self.head];
        entry.level = level;
        entry.timestamp = timestamp;
        let msg_bytes = message.as_bytes();
        let copy_len = msg_bytes.len().min(127);
        entry.message[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);
        entry.message[copy_len] = 0;
        entry.len = copy_len as u8;
        self.head = (self.head + 1) % LOG_BUFFER_CAPACITY;
        if self.count < LOG_BUFFER_CAPACITY {
            self.count += 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn drain_to<F>(&mut self, mut callback: F)
    where
        F: FnMut(u8, &[u8], u64),
    {
        let start = if self.count < LOG_BUFFER_CAPACITY { 0 } else { self.head };
        for i in 0..self.count {
            let idx = (start + i) % LOG_BUFFER_CAPACITY;
            let entry = &self.buffer[idx];
            let msg_end = entry.message.iter().position(|&b| b == 0).unwrap_or(entry.len as usize);
            callback(entry.level, &entry.message[..msg_end], entry.timestamp);
        }
        self.count = 0;
    }
}
