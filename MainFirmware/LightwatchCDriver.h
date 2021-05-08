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
    void fillScreen(uint16_t color);
    void serialPrintln(const char* text);
    uint8_t getTouch(int16_t &x, int16_t &y);
#ifdef __cplusplus
} //end extern "C"
#endif

#endif
