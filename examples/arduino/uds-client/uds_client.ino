#include <UDS.h>

UDSClient uds;

void setup() {
    Serial.begin(115200);
    uds.begin(&Serial);
    uds.onRpc("SetLed", onSetLed);
}

void loop() {
    uds.poll();
    delay(10);
}

void onSetLed(const uint8_t* data, size_t len) {
    if (len > 0 && data[0]) {
        digitalWrite(LED_BUILTIN, HIGH);
    } else {
        digitalWrite(LED_BUILTIN, LOW);
    }
}
