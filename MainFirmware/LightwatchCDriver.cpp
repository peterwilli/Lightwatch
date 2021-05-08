#include "LightwatchCDriver.h"

TTGOClass *ttgo;

void fillScreen(uint16_t color) {
    ttgo->tft->fillScreen(color);
}

void serialPrintln(const char* text) {
  Serial.println(text);
}

uint8_t getTouch(int16_t &x, int16_t &y) {
  return ttgo->getTouch(x, y) ? 1 : 0;
}

void initLightwatchCDriver() {
  ttgo = TTGOClass::getWatch();
  ttgo->begin();
  ttgo->openBL();
}
