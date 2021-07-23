#include <lightwatch_rust_blob.h>
#include "LightwatchCDriver.h"
#include <math.h>

void screenTest() {
    Serial.println("R");
    ttgo->tft->fillScreen(TFT_RED);
    delay(1000);
    Serial.println("G");
    ttgo->tft->fillScreen(TFT_GREEN);
    delay ;(1000);
    Serial.println("B");
    ttgo->tft->fillScreen(TFT_BLUE);
    delay(1000);
}

void setup()
{
    fminf(2, 3);
    Serial.begin(115200);
    initLightwatchCDriver();
//    screenTest();
    rust_bb_init();
}

void loop() {
    
}
