use uds_device::discovery::DiscoveryService;
use uds_device::traits::DeviceDiscovery;
use std::time::Duration;

pub fn run_devices(scan: bool, watch: bool) -> anyhow::Result<()> {
    let discovery = DiscoveryService::new();

    if watch {
        println!("Watching for devices (Ctrl+C to stop)...");
        discovery.watch(Box::new(|event| {
            match event {
                uds_device::traits::DeviceEvent::Discovered(info) => {
                    println!("  [+] Discovered: {} ({})", info.name, info.id.0);
                }
                uds_device::traits::DeviceEvent::Lost(id) => {
                    println!("  [-] Lost: {}", id.0);
                }
                uds_device::traits::DeviceEvent::Updated(info) => {
                    println!("  [~] Updated: {} ({})", info.name, info.id.0);
                }
            }
        }))?;
        return Ok(());
    }

    let devices = if scan {
        println!("Scanning for devices...");
        discovery.discover("serial", Duration::from_secs(3))?
    } else {
        discovery.discover("mock", Duration::from_secs(1))?
    };

    if devices.is_empty() {
        println!("No devices found.");
        println!("Try: uds devices --scan");
        return Ok(());
    }

    println!("Found {} device(s):\n", devices.len());
    for (i, d) in devices.iter().enumerate() {
        println!("  [{}.] {} ({})", i + 1, d.name, d.id.0);
        for hint in &d.transport_hints {
            println!("       {:?} -> {}", hint.transport_type, hint.address);
        }
        println!("       CPU: {}, Flash: {}KB, RAM: {}KB",
            d.capabilities.hardware.cpu_type,
            d.capabilities.hardware.flash_size_kb,
            d.capabilities.hardware.ram_size_kb,
        );
        println!();
    }

    Ok(())
}
