use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub log_level: Option<String>,
    pub metrics_enabled: Option<bool>,
    pub default_transport: Option<String>,
    pub default_baud: Option<u32>,
    pub timeout_ms: Option<u64>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            log_level: Some("info".into()),
            metrics_enabled: Some(false),
            default_transport: Some("serial".into()),
            default_baud: Some(115200),
            timeout_ms: Some(30000),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub transport: String,
    pub path: Option<String>,
    pub baud: Option<u32>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub auth_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub transport: Option<String>,
    pub device: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UdsConfig {
    #[serde(default)]
    pub global: GlobalConfig,

    #[serde(default)]
    pub devices: std::collections::HashMap<String, DeviceConfig>,

    #[serde(default)]
    pub profiles: std::collections::HashMap<String, ProfileConfig>,
}
