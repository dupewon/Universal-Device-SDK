# Plugin SDK

UDS supports third-party plugins via a stable C ABI.

## What Plugins Can Do

- Add new transports (LoRa, CAN bus, etc.)
- Add new device adapters
- Add new RPC methods
- Add new CLI subcommands
- Add dashboard visualizations

## ABI

Plugins export:

- `uds_plugin_abi_version()` — ABI version
- `uds_plugin_name()` — Plugin name
- `uds_plugin_version()` — Plugin version
- `uds_plugin_register()` — Registration entry point
- `uds_plugin_unregister()` — Cleanup

## Building a Plugin

See `plugins/example-plugin/` for a complete Rust + C template.

```bash
uds plugins install ./my-plugin.so
uds plugins list
```
