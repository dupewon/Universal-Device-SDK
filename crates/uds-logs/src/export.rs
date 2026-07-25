use crate::ingest::LogEntry;

pub enum ExportFormat {
    Json,
    Plain,
}

pub fn export_logs(entries: &[LogEntry], format: ExportFormat) -> String {
    match format {
        ExportFormat::Json => serde_json::to_string(entries).unwrap_or_default(),
        ExportFormat::Plain => entries.iter()
            .map(|e| format!("[{}] {}: {}", e.timestamp, e.target, e.message))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
