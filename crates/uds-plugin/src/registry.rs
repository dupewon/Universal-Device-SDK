use std::collections::HashMap;

pub struct PluginRegistry {
    plugins: HashMap<String, PluginInfo>,
}

struct PluginInfo {
    name: String,
    version: String,
    loaded: bool,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, version: &str) {
        self.plugins.insert(
            name.to_string(),
            PluginInfo {
                name: name.to_string(),
                version: version.to_string(),
                loaded: true,
            },
        );
    }

    pub fn unregister(&mut self, name: &str) {
        self.plugins.remove(name);
    }

    pub fn list(&self) -> Vec<(&str, &str, bool)> {
        self.plugins
            .values()
            .map(|p| (p.name.as_str(), p.version.as_str(), p.loaded))
            .collect()
    }
}
