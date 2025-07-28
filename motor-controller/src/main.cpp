#include <Arduino.h>

int incomingByte = 0;

void setup() {
  Serial.begin(9600);
  for (int i = 2; i <= 42; i++) {
    pinMode(i, OUTPUT);
    digitalWrite(i, HIGH);
  }
}

void loop() {
  // put your main code here, to run repeatedly:
  if (Serial.available() > 0) {
    incomingByte = Serial.read();
    digitalWrite(incomingByte - 32, LOW);
    delay(500);
    digitalWrite(incomingByte - 32, HIGH);
  }
}
