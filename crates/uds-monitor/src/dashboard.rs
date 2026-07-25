pub struct DashboardData {
    pub device_id: String,
    pub log_lines: Vec<String>,
    pub metrics: std::collections::HashMap<String, f64>,
}

impl DashboardData {
    pub fn new(device_id: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
            log_lines: Vec::new(),
            metrics: std::collections::HashMap::new(),
        }
    }
}
