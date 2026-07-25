use uds_core::UdsConfig;
use std::path::Path;

pub fn load_config(path: Option<&str>) -> UdsConfig {
    let config_path = path.unwrap_or("~/.uds/config.toml");
    if Path::new(config_path).exists() {
        let content = std::fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        UdsConfig::default()
    }
}
