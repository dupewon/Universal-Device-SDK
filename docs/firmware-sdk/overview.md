# Firmware SDK

The UDS Firmware SDK provides C libraries for embedded devices to speak the UDS protocol.

## Supported Targets

- ESP32 (ESP-IDF)
- ESP8266 (Non-OS SDK / RTOS)
- STM32 (HAL + CMSIS)
- RP2040 (Pico SDK)
- Arduino (AVR)

## SDK Components

- **Protocol Client**: Frame encode/decode, handshake, RPC
- **Transport Adapters**: UART, SPI, I2C, BLE
- **OTA Client**: Receive chunks, verify, write to flash
- **Logging**: Structured log submission
- **Metrics**: Local collection and periodic reporting

## Resource Requirements

- RAM: < 2 KB
- ROM: < 16 KB
- Event-driven, non-blocking design
