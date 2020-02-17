#include <Wire.h>

const int I2C_ADDR = 8;
const int LED_PINS [] = { 8, 9, 10, 11, 12 };
const int LED_COUNT = sizeof(LED_PINS) / sizeof(int);

// i2c buffer
String buff = "";

void setup() {
    // Configure pins
    for(int i = 0; i < LED_COUNT; i++)
        pinMode(LED_PINS[i], OUTPUT);

    // Set-up i2c
    Wire.begin(I2C_ADDR);
    Wire.onReceive(onReceive);

    // Set-up serial for debugging
    Serial.begin(9600);

    // Initialize LEDs
    setLeds(LOW);
}

void loop() {
    delay(100);
}

void onReceive(int count) {
    // Fill buffer
    while(Wire.available()) {
        int b = Wire.read();
        if(b == 0)
            continue;
        buff += (char) b;
    }

    // Handle each command
    int i;
    while((i = buff.indexOf('\n')) > -1) {
        onCommand(buff.substring(0, i));
        buff.remove(0, i + 1);
    }
}

void onCommand(String cmd) {
    // Debug received command
    Serial.println("CMD: '" + cmd + "'");

    // Handle commands
    if(cmd.startsWith("led "))
        onLedCommand(cmd.substring(4));
    else
        Serial.println("CMD '" + cmd + "' is unknown");
}

void onLedCommand(String cmd) {
    if(cmd == "reset")
        setLeds(LOW);
    else
        setLed(cmd[0] - '0', cmd[2] - '0');
}

void setLed(int i, bool level) {
    digitalWrite(LED_PINS[i], level);
}

void setLeds(bool level) {
    for(int i = 0; i < LED_COUNT; i++)
        setLed(i, level);
}
