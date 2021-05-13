#include "LightwatchCDriver.h"
#include <WiFi.h>

TTGOClass *ttgo;

void fillScreen(uint16_t color) {
    ttgo->tft->fillScreen(color);
}

void setTextColor(uint16_t c) {
  ttgo->tft->setTextColor(c);
}

int16_t drawString(const char *string, int32_t x, int32_t y, uint8_t font) {
  ttgo->tft->drawString(string, x, y, font);
}

void drawLine(int32_t xs, int32_t ys, int32_t xe, int32_t ye, uint32_t color) {
  ttgo->tft->drawLine(xs, ys, xe, ye, color);
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
  ttgo->tft->setTextFont(1);
}
