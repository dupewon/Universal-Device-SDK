# CLI Overview

The `uds` command is the primary interface to UDS. It uses a subcommand structure:

```
uds <global-flags> <subcommand> [subcommand-flags] [args]
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--config <path>` | Config file path (default: `~/.uds/config.toml`) |
| `--log-level <level>` | Log level: error, warn, info, debug, trace |
| `--output <format>` | Output format: human, json |
| `--transport <name>` | Transport to use: serial, tcp, udp, ws, ble, usb |
| `--device <id>` | Target device ID |

## Subcommands

| Command | Description |
|---------|-------------|
| `uds init` | Create a new UDS project |
| `uds devices` | Discover and list devices |
| `uds inspect` | Show device details |
| `uds doctor` | Run diagnostics |
| `uds logs` | Tail device logs |
| `uds monitor` | Real-time device monitoring |
| `uds flash` | Flash firmware |
| `uds update` | OTA firmware update |
| `uds benchmark` | Run performance benchmarks |
| `uds plugins` | Manage plugins |
| `uds rpc` | Invoke an RPC method |
| `uds fs` | Filesystem operations |
| `uds build` | Build firmware |
| `uds firmware` | Manage firmware images |
| `uds generate` | Generate code from IDL |
| `uds help` | Show help |
