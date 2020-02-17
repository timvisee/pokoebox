#include <Wire.h>

const int I2C_ADDR = 8;
const int LED_PINS [] = { 8, 9, 10, 11, 12 };
const int LED_COUNT = sizeof(LED_PINS) / sizeof(int);

// Input buffers
String i2c_buff = "";
String serial_buff = "";

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
    // Sink serial input to buffer
    while(Serial.available()) {
        int b = Serial.read();
        if(b == 0)
            continue;
        serial_buff += (char) b;
    }

    // Handle serial commands
    int i;
    while((i = serial_buff.indexOf('\n')) > -1) {
        onCommand(serial_buff.substring(0, i));
        serial_buff.remove(0, i + 1);
    }

    delay(100);
}

void onReceive(int count) {
    // Sink i2c input to buffer
    while(Wire.available()) {
        int b = Wire.read();
        if(b == 0)
            continue;
        i2c_buff += (char) b;
    }

    // Handle i2c commands
    int i;
    while((i = i2c_buff.indexOf('\n')) > -1) {
        onCommand(i2c_buff.substring(0, i));
        i2c_buff.remove(0, i + 1);
    }
}

void onCommand(String cmd) {
    // Debug received command
    Serial.println("CMD: '" + cmd + "'");

    // Handle commands
    if(cmd == "help") {
        Serial.println("HELP:");
        Serial.println("- help: show help");
        Serial.println("- led reset: reset all LEDs");
        Serial.println("- led <INDEX> <LEVEL>: set LED level");
    } else if(cmd.startsWith("led "))
        onLedCommand(cmd.substring(4));
    else
        Serial.println("CMD '" + cmd + "' is unknown, use 'help'");
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
