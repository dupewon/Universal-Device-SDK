# Universal Device SDK (UDS)

A cross-platform, CLI-first developer platform for embedded device development. UDS unifies development, testing, deployment, and monitoring across heterogeneous hardware architectures under a single command-line interface.

## Features

- **Unified CLI** — one tool for all your devices: ESP32, STM32, RP2040, Arduino, and more
- **Pluggable Transports** — serial, TCP, UDP, WebSocket, BLE, USB — same commands, any link
- **Binary Wire Protocol** — zero-copy, encrypted, versioned, constrained-device-friendly
- **OTA Updates** — over-the-air firmware updates with rollback safety
- **Real-Time Monitoring** — structured logs, metrics, telemetry streaming
- **Plugin System** — extend transports, device families, RPC methods, and CLI subcommands with stable ABI
- **Language Bindings** — Rust, C, C++, Python, Node.js, TypeScript, Go, C#, Java, Kotlin, Swift
- **Dashboard** — optional web GUI built on top of the CLI
- **Local First** — fully offline, cloud is optional

## Quick Start

```bash
# Install UDS
curl -fsSL https://raw.githubusercontent.com/dupewon/Universal-Device-SDK/main/scripts/install.sh | sh

# Discover devices
uds devices --scan

# Flash firmware
uds flash firmware.bin --device esp32-001

# Monitor logs
uds monitor --device esp32-001
```

### Windows (PowerShell)
```powershell
iex "& { $(irm https://raw.githubusercontent.com/dupewon/Universal-Device-SDK/main/scripts/install.ps1) }"
```

## Documentation

Full documentation is available in the [docs/](docs/) directory:

- [Getting Started](docs/getting-started.md)
- [CLI Reference](docs/cli/)
- [Protocol Specification](docs/protocols/)
- [Firmware SDK](docs/firmware-sdk/)
- [Plugin SDK](docs/plugin-sdk/)
- [Examples](docs/examples/)

## Architecture

```
┌─────────────────────────────────────────────────┐
│                    CLI (uds)                      │
├─────────────────────────────────────────────────┤
│  RPC Layer    │  Device Layer  │  Plugin System  │
├─────────────────────────────────────────────────┤
│  Protocol Layer (binary, encrypted, versioned)   │
├─────────────────────────────────────────────────┤
│  Transport Layer (serial, TCP, BLE, USB, ...)    │
├─────────────────────────────────────────────────┤
│              Embedded Device Firmware             │
└─────────────────────────────────────────────────┘
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions, coding standards, and PR process.

## License

Apache 2.0 — see [LICENSE](LICENSE).
