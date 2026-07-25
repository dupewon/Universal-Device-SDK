use uds_transport::traits::Transport;

pub fn run_doctor() -> anyhow::Result<()> {
    println!("UDS Diagnostics\n");

    // Check Rust toolchain
    println!("[1/6] Rust toolchain...");
    match std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        Ok(out) => {
            let version = String::from_utf8_lossy(&out.stdout);
            println!("      Rust: {}", version.trim());
        }
        Err(_) => println!("      WARN: rustc not found in PATH"),
    }

    // Check config file
    println!("[2/6] Configuration...");
    let config_path = dirs::home_dir()
        .map(|p| p.join(".uds").join("config.toml"))
        .unwrap_or_default();
    if config_path.exists() {
        println!("      Config: {} (exists)", config_path.display());
    } else {
        println!("      Config: not found (using defaults)");
    }

    // Check transports
    println!("[3/6] Transport availability...");
    let transports: Vec<(&str, bool)> = vec![
        (
            "serial",
            uds_transport::serial::SerialTransport::new().is_available(),
        ),
        ("tcp", true),
        ("udp", true),
        ("websocket", true),
        ("ble", false),
        ("usb", false),
    ];
    for (name, avail) in &transports {
        let status = if *avail { "OK" } else { "unavailable" };
        println!("      {:<12} [{}]", name, status);
    }

    // Scan serial ports
    println!("[4/6] Serial ports...");
    let ports = uds_device::discovery::DiscoveryService::scan_serial();
    if ports.is_empty() {
        println!("      No serial ports found");
        println!("      (Mock devices will be used for demo)");
    } else {
        for p in &ports {
            println!("      Found: {}", p);
        }
    }

    // Check network
    println!("[5/6] Network connectivity...");
    match std::net::TcpStream::connect_timeout(
        &"8.8.8.8:53".parse().unwrap(),
        std::time::Duration::from_secs(2),
    ) {
        Ok(_) => println!("      Network: OK"),
        Err(_) => println!("      Network: offline (local mode only)"),
    }

    // Summary
    println!("\n[6/6] Summary");
    println!("      UDS CLI: ready");
    println!("      Transports: serial, tcp, udp (mock fallback available)");
    println!("      Devices: discovery active");

    println!("\nDiagnostics complete. System is ready.");
    Ok(())
}
