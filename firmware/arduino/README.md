# UDS Firmware SDK — Arduino

## Requirements

- Arduino IDE or arduino-cli
- Supported boards: Arduino Uno, Mega, Nano, Due

## Installation

Copy `libraries/UDS/` to your Arduino `libraries` folder.

## Usage

```cpp
#include <UDS.h>

UDSClient client;

void setup() {
    Serial.begin(115200);
    client.begin(&Serial);
}

void loop() {
    client.poll();
}
```
