#include <Wire.h>

const int I2C_ADDR = 8;
const int LED_PINS [] = { 8, 9, 10, 11, 12 };

// Input buffer
String buff = "";

void setup() {
    // Configure pins
    for(int i = 0; i < sizeof(LED_PINS) / sizeof(int); i++)
        pinMode(LED_PINS[i], OUTPUT);

    // Set-up i2c
    Wire.begin(I2C_ADDR);
    Wire.onReceive(onReceive);
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
    if(cmd == "on") {
        digitalWrite(LED_PINS[0], HIGH); 
    } else if(cmd == "off") {
        digitalWrite(LED_PINS[0], LOW);
    }
}
