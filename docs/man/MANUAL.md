# UDS User Manual

## Installation

### Linux / macOS
```bash
curl -fsSL https://raw.githubusercontent.com/dupewon/Universal-Device-SDK/main/scripts/install.sh | sh
```

### Windows (PowerShell)
```powershell
iex "& { $(irm https://raw.githubusercontent.com/dupewon/Universal-Device-SDK/main/scripts/install.ps1) }"
```

### From Source (Rust)
```bash
cargo install uds-cli
```

## Getting Started

1. **Discover devices**: `uds devices --scan`
2. **Inspect a device**: `uds inspect --device esp32-001`
3. **Flash firmware**: `uds flash firmware.bin --device esp32-001`
4. **Monitor logs**: `uds monitor`
5. **Run diagnostics**: `uds doctor`

## Configuration

UDS looks for `~/.uds/config.toml`:

```toml
[global]
default_transport = "serial"
default_baud = 115200
timeout = 30000

[devices."esp32-001"]
transport = "serial"
path = "COM3"
baud = 115200
```

## Projects

Initialize a new project:
```bash
uds init
cd my-project
uds build
uds flash target/esp32/release/firmware.bin
```

## Shell Completions

```bash
# Bash
source completions/uds.bash

# Zsh
source completions/uds.zsh

# Fish
source completions/uds.fish
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Usage error |
| 3 | Device error |
| 4 | Network error |
| 5 | Configuration error |
