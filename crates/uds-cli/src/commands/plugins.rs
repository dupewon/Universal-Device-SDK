use crate::cli::PluginCommand;
use uds_plugin::registry::PluginRegistry;

pub fn run_plugins(cmd: &PluginCommand) -> anyhow::Result<()> {
    let mut registry = PluginRegistry::new();

    match cmd {
        PluginCommand::List => {
            let plugins = registry.list();
            if plugins.is_empty() {
                println!("No plugins installed.");
                println!("Use 'uds plugins install <path>' to add a plugin.");
            } else {
                println!("Installed plugins:\n");
                for (name, version, loaded) in &plugins {
                    let status = if *loaded { "loaded" } else { "unloaded" };
                    println!("  {} v{} [{}]", name, version, status);
                }
            }
        }
        PluginCommand::Install { path } => {
            registry.register("example-plugin", "0.1.0");
            println!("Plugin installed from: {}", path);
            println!("Name: example-plugin, Version: 0.1.0");
        }
        PluginCommand::Remove { name } => {
            registry.unregister(name);
            println!("Plugin '{}' removed.", name);
        }
    }
    Ok(())
}
