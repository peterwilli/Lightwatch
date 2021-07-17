FROM rust-xtensa:latest
MAINTAINER Peter Willemsen <peter@codebuffet.co>
RUN curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | BINDIR=/usr/bin sh
ENV ARDUINO_BOARD_MANAGER_ADDITIONAL_URLS=https://dl.espressif.com/dl/package_esp32_index.json
ENV ARDUINO_DIRECTORIES_USER=/etc/arduino
ENV ARDUINO_DIRECTORIES_DATA=/etc/arduino_data
RUN mkdir -p /etc/arduino && mkdir -p /etc/arduino_data
RUN arduino-cli core update-index && arduino-cli core install esp32:esp32
ARG ARDUINO_BOARD_PATCH_PATH=/etc/esp32_arduino_board_watch.patch
ADD ./esp32_arduino_board_watch.patch $ARDUINO_BOARD_PATCH_PATH
RUN patch /etc/arduino_data/packages/esp32/hardware/esp32/1.0.6/platform.txt < $ARDUINO_BOARD_PATCH_PATH && rm $ARDUINO_BOARD_PATCH_PATH
RUN mkdir -p /etc/arduino/libraries && ln -s /src/rust_blob_library /etc/arduino/libraries/rust_blob_library
RUN cd /etc/arduino/libraries && \
	wget https://github.com/Xinyuan-LilyGO/TTGO_TWatch_Library/archive/refs/tags/V1.4.2.tar.gz -O TTWatch_Lib.tar.gz && \
	tar -xvf TTWatch_Lib.tar.gz && \
	rm TTWatch_Lib.tar.gz
ADD ./lw /usr/bin