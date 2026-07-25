# Device Discovery Protocol

## Active Scan

UDS broadcasts a UDP multicast packet on port 4567, address `239.255.0.123`. Devices respond with their `DeviceInfo` structure.

## Passive Listen

The host listens on the multicast port for unsolicited device announcements (beacon frames).

## Serial / BLE Discovery

For serial and BLE transports, discovery scans available ports/advertisements and sends protocol handshake to identify UDS-capable devices.

## CLI Usage

```bash
# Scan for devices
uds devices --scan

# Watch for new devices
uds devices --watch
```
