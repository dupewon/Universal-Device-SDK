# UDS Firmware SDK — RP2040 (Raspberry Pi Pico)

## Requirements

- Raspberry Pi Pico SDK
- ARM GCC toolchain (`arm-none-eabi-gcc`)

## Build

```bash
cd firmware/rp2040
mkdir build && cd build
cmake ..
make
```

## Flashing

Hold BOOTSEL button and connect USB, then copy `uds_firmware.uf2` to the RPI-RP2 drive.
