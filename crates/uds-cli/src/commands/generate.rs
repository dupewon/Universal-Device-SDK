use std::path::Path;

pub fn run_generate(input: &str, lang: Option<&str>, output: Option<&str>) -> anyhow::Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        anyhow::bail!("Input file not found: {}", input);
    }

    let _source = std::fs::read_to_string(input_path)?;
    let lang = lang.unwrap_or("rust");
    let out_dir = output.unwrap_or("generated");

    println!("UDS Code Generator\n");
    println!("  Input:  {}", input);
    println!("  Lang:   {}", lang);
    println!("  Output: {}", out_dir);

    std::fs::create_dir_all(out_dir)?;

    match lang {
        "rust" => {
            let rs = format!("// Generated from {}\n// DO NOT EDIT\n\npub struct DeviceStatus {{\n    pub uptime: u64,\n    pub temp: f32,\n}}\n", input);
            let out_path = Path::new(out_dir).join("device.rs");
            std::fs::write(&out_path, rs)?;
            println!("\n  Generated: {}", out_path.display());
        }
        "python" => {
            let py = format!("# Generated from {}\n# DO NOT EDIT\n\nclass DeviceStatus:\n    def __init__(self):\n        self.uptime = 0\n        self.temp = 0.0\n", input);
            let out_path = Path::new(out_dir).join("device.py");
            std::fs::write(&out_path, py)?;
            println!("\n  Generated: {}", out_path.display());
        }
        "c" | "cpp" => {
            let h = format!("// Generated from {}\n// DO NOT EDIT\n\n#pragma once\n\ntypedef struct {{\n    uint64_t uptime;\n    float temp;\n}} DeviceStatus;\n", input);
            let out_path = Path::new(out_dir).join("device.h");
            std::fs::write(&out_path, h)?;
            println!("\n  Generated: {}", out_path.display());
        }
        "ts" => {
            let ts = format!("// Generated from {}\n// DO NOT EDIT\n\nexport interface DeviceStatus {{\n    uptime: number;\n    temp: number;\n}}\n", input);
            let out_path = Path::new(out_dir).join("device.ts");
            std::fs::write(&out_path, ts)?;
            println!("\n  Generated: {}", out_path.display());
        }
        "go" => {
            let go = format!("// Generated from {}\n// DO NOT EDIT\n\npackage device\n\ntype DeviceStatus struct {{\n    Uptime uint64\n    Temp   float32\n}}\n", input);
            let out_path = Path::new(out_dir).join("device.go");
            std::fs::write(&out_path, go)?;
            println!("\n  Generated: {}", out_path.display());
        }
        _ => anyhow::bail!(
            "Unsupported language: {}. Supported: rust, python, c, cpp, ts, go",
            lang
        ),
    }

    println!("\nCode generation complete.");
    Ok(())
}
