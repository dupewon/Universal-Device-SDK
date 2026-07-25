use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_logs(level: Option<&str>, _lines: Option<u32>) -> anyhow::Result<()> {
    let level_filter = level.unwrap_or("info");
    println!("UDS Logs (filter: {})", level_filter);
    println!("Monitoring device logs...\n");

    let sample_logs = vec![
        ("info", "System initialized, v0.1.0"),
        ("info", "WiFi connected to 'OfficeNet' (signal: -45 dBm)"),
        ("debug", "BLE advertisement started on channel 37"),
        ("info", "MQTT broker connected at 192.168.1.100:1883"),
        ("warn", "Flash wear leveling: block 12 reached 90% capacity"),
        (
            "info",
            "Sensor reading: temp=24.3°C, humidity=58%, pressure=1013.2hPa",
        ),
        ("error", "I2C bus timeout on address 0x76, retrying..."),
        ("info", "OTA update available: v0.2.0 (4.2 MB)"),
        ("debug", "RPC call 'GetStatus' handled in 2.3ms"),
        ("info", "Uptime: 3d 14h 22m 11s"),
    ];

    for (lvl, msg) in &sample_logs {
        let _ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let level_prio = match *lvl {
            "error" => 0,
            "warn" => 1,
            "info" => 2,
            "debug" => 3,
            "trace" => 4,
            _ => 5,
        };
        let filter_prio = match level_filter {
            "error" => 0,
            "warn" => 1,
            "info" => 2,
            "debug" => 3,
            "trace" => 4,
            _ => 2,
        };
        if level_prio > filter_prio {
            continue;
        }
        let color = match *lvl {
            "error" => "\x1b[31m",
            "warn" => "\x1b[33m",
            "info" => "\x1b[36m",
            "debug" => "\x1b[90m",
            _ => "\x1b[0m",
        };
        println!("{}[{:>5}] {}{}\x1b[0m", color, lvl, msg, color);
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    println!("\n(End of log buffer. Use --lines to show more.)");
    Ok(())
}
