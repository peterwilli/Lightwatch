#ifndef LIGHTWATCH_C_DRIVER_H
#define LIGHTWATCH_C_DRIVER_H

#include <stdint.h>

#ifdef ARDUINO
#include "config.h"
extern TTGOClass *ttgo;
void initLightwatchCDriver();
#endif // end Arduino

#ifdef __cplusplus
extern "C" {
#endif
    // System stuff
    void serialPrintln(const char* text);
    void setBrightness(uint8_t brightness);
        
    // Drawing
    void fillScreen(uint16_t color);
    void setTextColor(uint16_t c);    
    int16_t drawString(const char *string, int32_t x, int32_t y, uint8_t font);  // Draw string using specifed font number
    void setTextDatum(uint8_t datum);
    void drawLine(int32_t xs, int32_t ys, int32_t xe, int32_t ye, uint32_t color);         
    void pushImage(int32_t x0, int32_t y0, int32_t w, int32_t h, uint16_t *data);
    //End Drawing

    uint8_t getTouch(int16_t &x, int16_t &y);
             
#ifdef __cplusplus
} //end extern "C"
#endif

#endif
