use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "uds", about = "Universal Device SDK CLI - Unified embedded development toolchain", version, long_about = None)]
pub struct UdsCli {
    #[arg(long, help = "Config file path", default_value = "~/.uds/config.toml")]
    pub config: String,

    #[arg(
        long,
        help = "Log level [error, warn, info, debug, trace]",
        default_value = "info"
    )]
    pub log_level: String,

    #[arg(long, help = "Output format [human, json]", default_value = "human")]
    pub output: String,

    #[arg(long, help = "Transport to use [serial, tcp, udp, ws, ble, usb, mock]")]
    pub transport: Option<String>,

    #[arg(long, help = "Target device ID")]
    pub device: Option<String>,

    #[command(subcommand)]
    pub command: Option<UdsCommand>,
}

#[derive(Subcommand)]
pub enum UdsCommand {
    /// Initialize a new UDS project in the current directory
    Init,

    /// List and discover connected devices
    Devices {
        #[arg(long, short, help = "Scan for new devices")]
        scan: bool,

        #[arg(long, help = "Watch for device changes continuously")]
        watch: bool,
    },

    /// Show detailed device information
    Inspect {
        #[arg(help = "Device ID to inspect")]
        device_id: Option<String>,
    },

    /// Run system diagnostics
    Doctor,

    /// Tail device logs with optional filtering
    Logs {
        #[arg(long, help = "Filter by log level [error, warn, info, debug, trace]")]
        level: Option<String>,

        #[arg(long, short, help = "Number of lines to show")]
        lines: Option<u32>,
    },

    /// Real-time device monitoring (logs + metrics)
    Monitor,

    /// Flash firmware image to device
    Flash {
        #[arg(help = "Path to firmware image file (.bin, .hex, .elf)")]
        firmware: String,

        #[arg(long, help = "Verify image after flashing")]
        verify: bool,

        #[arg(long, help = "Perform OTA update instead of direct flash")]
        ota: bool,

        #[arg(long, help = "Target partition")]
        partition: Option<String>,
    },

    /// OTA firmware update
    Update {
        #[arg(help = "Path to firmware image file")]
        firmware: String,

        #[arg(long, help = "Roll back to previous firmware")]
        rollback: bool,
    },

    /// Run performance benchmarks on connected device
    Benchmark {
        #[arg(
            long,
            help = "Type of benchmark [latency, throughput, all]",
            default_value = "all"
        )]
        kind: String,
    },

    /// Manage UDS plugins
    Plugins {
        #[command(subcommand)]
        command: PluginCommand,
    },

    /// Invoke an RPC method manually
    Rpc {
        #[arg(help = "Method name to call")]
        method: String,

        #[arg(help = "JSON-encoded parameters")]
        params: Option<String>,

        #[arg(long, help = "Device ID to target")]
        device_id: Option<String>,
    },

    /// Filesystem operations on device
    Fs {
        #[command(subcommand)]
        command: FsCommand,
    },

    /// Build firmware from source
    Build {
        #[arg(help = "Path to firmware source directory")]
        path: Option<String>,

        #[arg(long, help = "Target device type [esp32, stm32, rp2040, arduino]")]
        target: Option<String>,
    },

    /// Manage firmware images
    Firmware {
        #[command(subcommand)]
        command: FirmwareCommand,
    },

    /// Generate code from IDL definitions
    Generate {
        #[arg(help = "Path to .uds IDL file")]
        input: String,

        #[arg(long, help = "Output language [rust, c, cpp, python, ts, go]")]
        lang: Option<String>,

        #[arg(long, help = "Output directory")]
        output: Option<String>,
    },

    /// Open documentation
    Docs,

    /// Print version information
    Version,
}

#[derive(Subcommand)]
pub enum PluginCommand {
    /// List installed plugins
    List,
    /// Install a plugin from path
    Install { path: String },
    /// Remove a plugin
    Remove { name: String },
}

#[derive(Subcommand)]
pub enum FsCommand {
    /// List directory contents
    Ls { path: String },
    /// Read and display file contents
    Cat { path: String },
    /// Copy file to/from device
    Cp { source: String, dest: String },
    /// Move/rename file
    Mv { source: String, dest: String },
    /// Remove file
    Rm { path: String },
    /// Create directory
    Mkdir { path: String },
}

#[derive(Subcommand)]
pub enum FirmwareCommand {
    /// List available firmware images
    List,
    /// Verify firmware image integrity
    Verify { path: String },
    /// Sign firmware image
    Sign { path: String, key: String },
}

impl UdsCli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
