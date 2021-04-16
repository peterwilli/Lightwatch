#include "LightwatchCDriver.h"

TTGOClass *ttgo;

void fillScreen(uint16_t color) {
    ttgo->tft->fillScreen(color);
}

void initLightwatchCDriver() {
  ttgo = TTGOClass::getWatch();
  ttgo->begin();
  ttgo->openBL();
}
