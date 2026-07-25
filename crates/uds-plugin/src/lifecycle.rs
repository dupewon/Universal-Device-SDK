pub enum PluginState {
    Unloaded,
    Loading,
    Loaded,
    Error(String),
}

pub struct PluginLifecycle {
    state: PluginState,
}

impl Default for PluginLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLifecycle {
    pub fn new() -> Self {
        Self {
            state: PluginState::Unloaded,
        }
    }

    pub fn state(&self) -> &PluginState {
        &self.state
    }

    pub fn transition(&mut self, new_state: PluginState) {
        self.state = new_state;
    }
}
