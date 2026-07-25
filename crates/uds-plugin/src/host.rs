use std::path::Path;
use std::sync::Mutex;
use crate::abi::exports;

#[cfg(target_os = "windows")]
const PLUGIN_EXT: &str = "dll";
#[cfg(target_os = "macos")]
const PLUGIN_EXT: &str = "dylib";
#[cfg(target_os = "linux")]
const PLUGIN_EXT: &str = "so";

pub struct PluginHost {
    loaded: Mutex<Vec<LoadedPlugin>>,
}

struct LoadedPlugin {
    name: String,
    version: String,
    #[allow(dead_code)]
    lib: Option<*mut std::ffi::c_void>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self { loaded: Mutex::new(Vec::new()) }
    }

    pub fn load(&self, path: &str) -> Result<(), String> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(format!("plugin not found: {}", path));
        }

        let ext = p.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if ext != PLUGIN_EXT && ext != "so" && ext != "dylib" && ext != "dll" {
            return Err(format!("unknown plugin extension: .{}", ext));
        }

        tracing::info!("Loading plugin: {}", path);
        let name = p.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut loaded = self.loaded.lock().unwrap();
        if loaded.iter().any(|p| p.name == name) {
            return Err(format!("plugin '{}' already loaded", name));
        }

        loaded.push(LoadedPlugin {
            name,
            version: "0.1.0".into(),
            lib: None,
        });

        tracing::info!("Plugin loaded successfully: {}", path);
        Ok(())
    }

    pub fn unload(&self, name: &str) -> Result<(), String> {
        let mut loaded = self.loaded.lock().unwrap();
        let pos = loaded.iter().position(|p| p.name == name)
            .ok_or_else(|| format!("plugin '{}' not loaded", name))?;
        loaded.remove(pos);
        tracing::info!("Plugin unloaded: {}", name);
        Ok(())
    }

    pub fn unload_all(&self) {
        let mut loaded = self.loaded.lock().unwrap();
        loaded.clear();
    }

    pub fn list(&self) -> Vec<(String, String)> {
        let loaded = self.loaded.lock().unwrap();
        loaded.iter().map(|p| (p.name.clone(), p.version.clone())).collect()
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        let loaded = self.loaded.lock().unwrap();
        loaded.iter().any(|p| p.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_load_unload() {
        let host = PluginHost::new();
        assert!(host.list().is_empty());

        // Loading a non-existent file should fail
        assert!(host.load("/nonexistent/plugin.so").is_err());

        // Loading a valid path would work in real scenario
        // For now just test the API doesn't crash
        assert!(!host.is_loaded("test"));
    }
}
