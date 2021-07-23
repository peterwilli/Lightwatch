#ifndef LIGHTWATCH_C_DRIVER_H
#define LIGHTWATCH_C_DRIVER_H

#include <stdint.h>

#ifdef ARDUINO
#include "config.h"
extern TTGOClass *ttgo;
void initLightwatchCDriver();
#else
struct bma4_accel {
    /*! Accel X data */
    int16_t x;
    /*! Accel Y data */
    int16_t y;
    /*! Accel Z data */
    int16_t z;
};
typedef struct bma4_accel Accel;
#endif // end Arduino

struct rtc_date {
    uint16_t year;
    uint8_t month;
    uint8_t day;
    uint8_t hour;
    uint8_t minute;
    uint8_t second;
};
typedef struct rtc_date RTCDate;

#ifdef __cplusplus
extern "C" {
#endif
    // System stuff
    void enableAccelerometer();
    uint8_t readAccelerometer(Accel &accel);
    void serialPrintln(const char* text);
    void setBrightness(uint8_t brightness);
    void getScreenSize(uint16_t &w, uint16_t &h);
    uint8_t readIRQ();

    // Utils
    uint16_t color565(uint8_t r, uint8_t g, uint8_t b);

    // Pin numbers (Idk how else to do this)
    uint8_t getPinAXP202();
        
    // Vibrator
    void vibrate(uint8_t duration);
    void enableVibrator();

    // RTC
    void enableRTC();
    void rtc_setDateTime(uint16_t year,
                     uint8_t month,
                     uint8_t day,
                     uint8_t hour,
                     uint8_t minute,
                     uint8_t second);
    void rtc_getDateTime(RTCDate &rtcDate);
    
    
    // Power management
    void displaySleep();
    void displayWakeup();
    void powerOffEverythingExceptESP32();
    uint8_t getRTCDataAtIndex(uint16_t index);
    void setRTCDataAtIndex(uint16_t index, uint8_t data);
    void deepSleep(uint32_t sleepMillis);

    // Drawing
    void setTextSize(uint8_t size);
    void fillRect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);
    void fillScreen(uint16_t color);
    void setTextColor(uint16_t c);    
    int16_t drawString(const char *string, int32_t x, int32_t y, uint8_t font);  // Draw string using specifed font number
    void setTextDatum(uint8_t datum);
    void drawLine(int32_t xs, int32_t ys, int32_t xe, int32_t ye, uint32_t color);         
    void pushImage(int32_t x0, int32_t y0, int32_t w, int32_t h, uint16_t *data);

    void tft_setAddrWindow(int32_t x0, int32_t y0, int32_t w, int32_t h);
    void tft_startWrite();
    void tft_endWrite();
    void tft_pushColor(uint16_t color);
    //End Drawing

    uint8_t getTouch(int16_t &x, int16_t &y);
             
#ifdef __cplusplus
} //end extern "C"
#endif

#endif
