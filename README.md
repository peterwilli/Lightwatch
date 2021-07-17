# Lightwatch

The coolest watch in the world.

---

The Lightwatch firmware is a Rust binding for the [TTGO T-Watch](https://github.com/Xinyuan-LilyGO/TTGO_TWatch_Library) library.

It's meant to sideload Rust on the C++-based firmware, so that one can use the Rust language on the T-Watch without having to port the library and it's drivers to Rust.

But it will become a fully-fledged firmware, giving both techies a way to customize with ease, yet giving normal users a way to use it "out of the box".

## Development

See [Our Trello board](https://trello.com/b/fGMwgs0I/development)

## How to compile

We now streamlined our build-sequence in a reproducable set of Docker images. The first image is the bare minimum to compile Rust on ESP32. This one you can also use for your own projects.

The second one has development scripts specifically for the Lightwatch firmware, building the blob and push it to the device.

With the instructions below, I assume you either run Linux with a working Docker installation. It should work on other operating systems, but is untested.

Text `like this` are commands!

- Start by cloning this repo: `git clone https://github.com/peterwilli/Lightwatch.git`
- `cd` to the cloned directory.
- Make an alias to the Docker image for convenience: `alias lw='docker run --rm -it -v "$(pwd):/src" -v "$(pwd)/.arduino_tmp:/tmp/arduino" --privileged peterwilli/lightwatch-dev-env:latest lw'`.

  - In the next steps, be sure to run `lw` in this repo's directory!

- You can now run any of the following (For building and upload):

  - `lw build-rust-blob` <- Builds the rust binary blob and automatically copies it over to the Arduino project.
  - `lw upload` <- Compiles the Arduino project, and uploads the compiled project to your device if it's plugged in. The first time you run this you will be asked to provide a path to the plugged in device. Be sure to follow the output of the upload command.
  - `lw build-arduino` <- Compiles the Arduino project.

  For a simple build and upload to your device, you only have to run the first 2 commands.

- Good luck, it should run!

  If you have any questions, you can reach me on the [Tanglehub Discord](https://discord.gg/wwnhaRas2N)
