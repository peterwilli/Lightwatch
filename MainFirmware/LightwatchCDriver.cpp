#include "LightwatchCDriver.h"
#include <WiFi.h>

TTGOClass *ttgo;
bool irq = false;
bool bma423Irq = false; 
bool rtcIrq = false;
RTC_DATA_ATTR uint8_t RTC_DATA[1024] = {0};
#define uS_TO_mS_FACTOR 1000  /* Conversion factor for micro seconds to miliseconds */
#define COLOR565(r,g,b)  ((r & 0xF8) << 8) | ((g & 0xFC) << 3) | (b >> 3)

uint8_t getRTCDataAtIndex(uint16_t index) {
  return RTC_DATA[index];
}

uint16_t color565(uint8_t r, uint8_t g, uint8_t b) {
  return COLOR565(r, g, b);
}

void setRTCDataAtIndex(uint16_t index, uint8_t data) {
  RTC_DATA[index] = data;
}

void setDisplayState(bool displayOn) {
  ttgo->power->setPowerOutPut(AXP202_LDO2, displayOn);
}

void deepSleep(uint32_t sleepMillis) {
  // Set screen and touch to sleep mode
  ttgo->displaySleep();

  /*
  When using T - Watch2020V1, you can directly call power->powerOff(),
  if you use the 2019 version of TWatch, choose to turn off
  according to the power you need to turn off
  */
#ifdef LILYGO_WATCH_2020_V1
  ttgo->powerOff();
  // LDO2 is used to power the display, and LDO2 can be turned off if needed
  // power->setPowerOutPut(AXP202_LDO2, false);
#else
  ttgo->power->setPowerOutPut(AXP202_LDO3, false);
  ttgo->power->setPowerOutPut(AXP202_LDO4, false);
  ttgo->power->setPowerOutPut(AXP202_LDO2, false);
  // The following power channels are not used
  ttgo->power->setPowerOutPut(AXP202_EXTEN, false);
  ttgo->power->setPowerOutPut(AXP202_DCDC2, false);
#endif

  esp_sleep_enable_timer_wakeup(((uint64_t) sleepMillis) * uS_TO_mS_FACTOR);
  esp_deep_sleep_start();
}

void rtc_getDateTime(RTCDate &rtcDate) {
  RTC_Date curr_datetime = ttgo->rtc->getDateTime();
  rtcDate.year = curr_datetime.year;
  rtcDate.month = curr_datetime.month;
  rtcDate.day = curr_datetime.day;
  rtcDate.hour = curr_datetime.hour;
  rtcDate.minute = curr_datetime.minute;
  rtcDate.second = curr_datetime.second;
}

void rtc_setDateTime(uint16_t year,
                     uint8_t month,
                     uint8_t day,
                     uint8_t hour,
                     uint8_t minute,
                     uint8_t second) {
  ttgo->rtc->setDateTime(year, month, day, hour, minute, second);
}

void enableRTC() {
  pinMode(RTC_INT_PIN, INPUT_PULLUP);
  attachInterrupt(RTC_INT_PIN, [] {
      rtcIrq = 1;
  }, FALLING);
}

void enableVibrator() {
  ttgo->motor_begin();
}

void powerOffEverythingExceptESP32() {
  ttgo->powerOff();
}

void displayOff() {
  ttgo->displayOff();
}    

void displaySleep() {
  ttgo->displaySleep();
  ttgo->power->setPowerOutPut(AXP202_LDO2, false);
}

void displayWakeup() {
  ttgo->displayWakeup();
  ttgo->power->setPowerOutPut(AXP202_LDO2, true);
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

void tft_pushColor(uint16_t color) {
  ttgo->tft->pushColor(color);
}

void tft_setAddrWindow(int32_t x0, int32_t y0, int32_t w, int32_t h) {
  ttgo->tft->setAddrWindow(x0, y0, w, h);
}

void tft_startWrite() {
  ttgo->tft->startWrite();
}

void tft_endWrite() {
  ttgo->tft->endWrite();
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

  pinMode(BMA423_INT1, INPUT);
  attachInterrupt(BMA423_INT1, [] {
      // Set interrupt to set irq value to 1
      bma423Irq = 1;
  }, RISING); //It must be a rising edge

  ttgo->bma->accelConfig(cfg);
  ttgo->bma->enableAccel();
}

void enableStepCounter() {
  // Enable BMA423 accelerometer
  // Warning : To use steps, you must first enable the accelerometer
  ttgo->bma->enableAccel();
  
  // Enable BMA423 step count feature
  ttgo->bma->enableFeature(BMA423_STEP_CNTR, true);
  ttgo->bma->resetStepCounter();
}

uint32_t getStepCount() {
  return ttgo->bma->getCounter();   
}

void getScreenSize(uint16_t &w, uint16_t &h) {
  w = TFT_WIDTH;
  h = TFT_HEIGHT;
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