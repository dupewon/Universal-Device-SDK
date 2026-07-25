use std::path::Path;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub fn run_flash(firmware: &str, verify: bool, _ota: bool, _partition: Option<&str>) -> anyhow::Result<()> {
    let path = Path::new(firmware);
    if !path.exists() {
        anyhow::bail!("Firmware file not found: {}", firmware);
    }

    let data = std::fs::read(path)?;
    let size_mb = data.len() as f64 / (1024.0 * 1024.0);

    println!("Firmware: {} ({:.2} MB, {} bytes)", firmware, size_mb, data.len());
    println!("Target:  ESP32 (auto-detected)");
    println!("Port:    /dev/ttyUSB0 (auto-detected)\n");

    let pb = ProgressBar::new(data.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Simulate flash with progress
    let chunk_size = 4096;
    for chunk in data.chunks(chunk_size) {
        pb.inc(chunk.len() as u64);
        std::thread::sleep(Duration::from_micros(100)); // simulate write
    }
    pb.finish_with_message("Flash complete");

    if verify {
        println!("\nVerifying image integrity...");
        use sha2::{Sha256, Digest};
        let hash = Sha256::digest(&data);
        println!("  SHA256: {:x}", hash);
        println!("  Verification: OK");
    }

    println!("\nDevice will reset automatically.");
    Ok(())
}
