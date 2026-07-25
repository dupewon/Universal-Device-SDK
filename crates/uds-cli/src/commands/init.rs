use std::path::Path;

pub fn run_init() -> anyhow::Result<()> {
    let dirs = [
        "src",
        "firmware",
        "configs",
        "scripts",
        "tests",
    ];

    for d in &dirs {
        let path = Path::new(d);
        if !path.exists() {
            std::fs::create_dir_all(path)?;
            println!("  Created: {}/", d);
        }
    }

    let uds_toml = r#"[project]
name = "my-uds-project"
version = "0.1.0"

[device]
default_transport = "serial"
default_baud = 115200
"#;

    if !Path::new("uds.toml").exists() {
        std::fs::write("uds.toml", uds_toml)?;
        println!("  Created: uds.toml");
    }

    if !Path::new(".gitignore").exists() {
        std::fs::write(".gitignore", "target/\n*.bin\n*.hex\n*.elf\n")?;
        println!("  Created: .gitignore");
    }

    println!("\nUDS project initialized successfully.");
    println!("Run `uds devices --scan` to discover connected devices.");
    Ok(())
}
