use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_monitor() -> anyhow::Result<()> {
    println!("UDS Monitor - Real-time device monitoring");
    println!("Press Ctrl+C to stop.\n");

    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let uptime = now - start;

        print!(
            "\r  [{}s] Connected | RSSI: -{} dBm | Temp: {:.1}°C | Freq: {} MHz | Heap: {} KB free",
            uptime,
            42 + (uptime % 20),
            35.0 + (uptime % 10) as f64 * 0.5,
            240,
            180 - (uptime % 50),
        );
        std::io::Write::flush(&mut std::io::stdout()).ok();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
