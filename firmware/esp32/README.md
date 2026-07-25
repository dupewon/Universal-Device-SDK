# UDS Firmware SDK — ESP32

## Requirements

- ESP-IDF v5.0+
- Rust toolchain with `xtensa-esp32-espidf` target

## Build

```bash
cd firmware/esp32
idf.py set-target esp32
idf.py build
idf.py flash
```

## Components

- `components/uds/` — UDS protocol client library
- `main/` — Example firmware with UDS server
