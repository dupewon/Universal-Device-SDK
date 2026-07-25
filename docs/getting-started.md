# Getting Started with UDS

## Installation

### Linux / macOS

```bash
curl -fsSL https://universal-device-sdk.dev/install.sh | sh
```

### Windows

```powershell
winget install universal-device-sdk
```

### From Source

```bash
git clone https://github.com/universal-device-sdk/uds.git
cd uds
cargo build --release
./target/release/uds --help
```

## First Steps

1. **Discover devices**: `uds devices --scan`
2. **Inspect a device**: `uds inspect --device <id>`
3. **Flash firmware**: `uds flash firmware.bin --device <id>`
4. **Monitor logs**: `uds monitor --device <id>`

## Configuration

UDS uses a TOML config file at `~/.uds/config.toml`:

```toml
[global]
default_transport = "serial"
default_baud = 115200

[devices."esp32-001"]
transport = "serial"
path = "/dev/ttyUSB0"
```
