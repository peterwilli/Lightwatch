#include "LightwatchCDriver.h"
#include <WiFi.h>

TTGOClass *ttgo;
bool irq = false;

void enableVibrator() {
  ttgo->motor_begin();
}

void vibrate(uint8_t duration) {
  ttgo->motor->onec(duration);
}

void setBrightness(uint8_t brightness) {
    ttgo->bl->adjust(brightness);
}

void fillScreen(uint16_t color) {
    ttgo->tft->fillScreen(color);
}

void fillRect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color) {
  ttgo->tft->fillRect(x, y, w, h, color);
}

void setTextColor(uint16_t c) {
  ttgo->tft->setTextColor(c);
}

void setTextSize(uint8_t size) {
  ttgo->tft->setTextSize(size);
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

uint8_t readAccelerometer(Accel &accel) {
  return ttgo->bma->getAccel((Accel&) accel) ? 1 : 0;
}

void enableAccelerometer() {
  // Accel parameter structure
  Acfg cfg;
  /*!
      Output data rate in Hz, Optional parameters:
          - BMA4_OUTPUT_DATA_RATE_0_78HZ
          - BMA4_OUTPUT_DATA_RATE_1_56HZ
          - BMA4_OUTPUT_DATA_RATE_3_12HZ
          - BMA4_OUTPUT_DATA_RATE_6_25HZ
          - BMA4_OUTPUT_DATA_RATE_12_5HZ
          - BMA4_OUTPUT_DATA_RATE_25HZ
          - BMA4_OUTPUT_DATA_RATE_50HZ
          - BMA4_OUTPUT_DATA_RATE_100HZ
          - BMA4_OUTPUT_DATA_RATE_200HZ
          - BMA4_OUTPUT_DATA_RATE_400HZ
          - BMA4_OUTPUT_DATA_RATE_800HZ
          - BMA4_OUTPUT_DATA_RATE_1600HZ
  */
  cfg.odr = BMA4_OUTPUT_DATA_RATE_100HZ;
  /*!
      G-range, Optional parameters:
          - BMA4_ACCEL_RANGE_2G
          - BMA4_ACCEL_RANGE_4G
          - BMA4_ACCEL_RANGE_8G
          - BMA4_ACCEL_RANGE_16G
  */
  cfg.range = BMA4_ACCEL_RANGE_2G;
  /*!
      Bandwidth parameter, determines filter configuration, Optional parameters:
          - BMA4_ACCEL_OSR4_AVG1
          - BMA4_ACCEL_OSR2_AVG2
          - BMA4_ACCEL_NORMAL_AVG4
          - BMA4_ACCEL_CIC_AVG8
          - BMA4_ACCEL_RES_AVG16
          - BMA4_ACCEL_RES_AVG32
          - BMA4_ACCEL_RES_AVG64
          - BMA4_ACCEL_RES_AVG128
  */
  cfg.bandwidth = BMA4_ACCEL_NORMAL_AVG4;

  /*! Filter performance mode , Optional parameters:
      - BMA4_CIC_AVG_MODE
      - BMA4_CONTINUOUS_MODE
  */
  cfg.perf_mode = BMA4_CONTINUOUS_MODE;
  
  ttgo->bma->accelConfig(cfg);
  ttgo->bma->enableAccel();
}

uint8_t getTouch(int16_t &x, int16_t &y) {
  return ttgo->getTouch(x, y) ? 1 : 0;
}

uint8_t getPinAXP202() {
  return AXP202_INT;
}

void initLightwatchCDriver() {
  ttgo = TTGOClass::getWatch();
  ttgo->begin();
  ttgo->openBL();
  ttgo->tft->setTextFont(1);

  pinMode(AXP202_INT, INPUT_PULLUP);
  attachInterrupt(AXP202_INT, [] {
      irq = true;
  }, FALLING);

  //!Clear IRQ unprocessed  first
  ttgo->power->enableIRQ(AXP202_PEK_SHORTPRESS_IRQ | AXP202_VBUS_REMOVED_IRQ | AXP202_VBUS_CONNECT_IRQ | AXP202_CHARGING_IRQ, true);
  ttgo->power->clearIRQ();
}

uint8_t readIRQ() {
  uint8_t result = 0;
  if(irq) {
    irq = false;
    ttgo->power->readIRQ();
    if(ttgo->power->isPEKShortPressIRQ()) {
      result = 1;
    }
    ttgo->power->clearIRQ();
  }
  return result;
}