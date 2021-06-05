#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct bma4_accel {
  /// Accel X data
  int16_t x;
  /// Accel Y data
  int16_t y;
  /// Accel Z data
  int16_t z;
};

using Accel = bma4_accel;

extern "C" {

void rust_bb_init();

extern uint8_t *malloc(uintptr_t size);

extern void free(uint8_t *ptr);

extern uintptr_t write(intptr_t file, const uint8_t *buffer, uintptr_t count);

extern void enableAccelerometer();

extern uint8_t readAccelerometer(Accel *accel);

extern void serialPrintln(const char *text);

extern void setBrightness(uint8_t brightness);

extern uint8_t readIRQ();

extern uint8_t getPinAXP202();

extern void setTextSize(uint8_t size);

extern void fillRect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);

extern void fillScreen(uint16_t color);

extern void setTextColor(uint16_t c);

extern int16_t drawString(const char *string, int32_t x, int32_t y, uint8_t font);

extern void setTextDatum(uint8_t datum);

extern void drawLine(int32_t xs, int32_t ys, int32_t xe, int32_t ye, uint32_t color);

extern void pushImage(int32_t x0, int32_t y0, int32_t w, int32_t h, uint16_t *data);

extern uint8_t getTouch(int16_t *x, int16_t *y);

extern uint32_t millis();

extern void delay(uint32_t ms);

extern uint8_t digitalRead(uint8_t pinNum);

} // extern "C"
