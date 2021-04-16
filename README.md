# Lightwatch

The coolest watch in the world.

----

The Lightwatch firmware is a Rust binding for the [TTGO T-Watch](https://github.com/Xinyuan-LilyGO/TTGO_TWatch_Library) library.

It's meant to sideload Rust on the C++-based firmware, so that one can use the Rust language on the T-Watch without having to port the library and it's drivers to Rust.

But it will become a fully-fledged firmware, giving both techies a way to customize with ease, yet giving normal users a way to use it "out of the box".

## Development

See [Our Trello board](https://trello.com/b/fGMwgs0I/development)

## How to compile

- You need the Xtensa patches for the Rust compiler, and the ESP32 target.

    - Thanks to the awesome work by @MabezDev and Xtensa, this is doable. Follow the [tutorial here](https://github.com/MabezDev/xtensa-rust-quickstart#recommended-build-method).

    - If you place the folders "xtensa-esp32-elf" and "rust-xtensa" in the root of this repo, you don't need to set PATH and make a setenv, just do `source ./setenv` inside the "rust_firmware" folder.

- Run `./compile_blob` inside the "rust_firmware" folder.
- You end up with a "liblightwatch_firmware.a" in "rust_firmware/target/xtensa-esp32-none-elf/debug". This file is automatically copied to the Arduino library.
- Copy or symlink the library "rust_blob_library" to your Arduino libraries folder.
- In Arduino, open the sketch "MainFirmware", and select the T-Watch board after installing the ESP32 board (see instructions [here](https://github.com/Xinyuan-LilyGO/TTGO_TWatch_Library/blob/master/docs/arduino-ide/boards_manager.md))

    - **Note**: At the time of writing this README (ESP32 board v1.0.6), there seems to be a bug in the ESP32 compiler for Arduino that messes up the library order, causing false "undefined reference" errors. I'm not sure if it's a real bug or not, so I went ahead and made a independent patch. To execute this patch run: `patch '/path/to/.arduino15/packages/esp32/hardware/esp32/1.0.6/platform.txt' < '/path/to/LightWatch/extras/esp32_arduino_board_watch.patch'`

- Good luck, it should run!
- If you have any questions, you can reach me on the [Tanglehub Discord](https://discord.gg/wwnhaRas2N)