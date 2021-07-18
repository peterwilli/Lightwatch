FROM debian:buster-slim
MAINTAINER Peter Willemsen <peter@codebuffet.co>
CMD ["bash"]
ENV BUILD_DIR=/var/lib/build
WORKDIR $BUILD_DIR

RUN echo "Installing dependencies" && \
	apt-get update  \
	&& apt-get install -y --no-install-recommends python-dev python-wheel cython python-setuptools git wget make libncurses-dev flex bison gperf python python-pip python-serial build-essential cmake libssl-dev python ninja-build ca-certificates curl netbase wget  \
	&& apt-get dist-upgrade -y \
	&& rm -rf /var/lib/apt/lists/*

RUN echo "Installing Rustup and Cargo..." && \
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh && \
	chmod +x ./rustup.sh && \
	./rustup.sh  --default-toolchain 1.52.0 --profile minimal -y && \ 
	rm rustup.sh

ENV PATH=$PATH:/root/.cargo/bin
RUN cargo install cargo-xbuild

ENV XTENSA_RUSTC_ARCHIVE_PATH=/rust-xtensa-precompiled.tar.xz

RUN echo "Downloading rust-xtensa source.." && \
	git clone https://github.com/MabezDev/rust-xtensa $BUILD_DIR/rust-xtensa && \
	cd $BUILD_DIR/rust-xtensa && \
	git reset --hard c4c02e8375fccb31e5d0e0e1bf283e40413fe59b && \
	git submodule update --init --recursive && \
	rm -rf ./.git && \
	echo "Compiling Rust with the Xtensa patches... This is gonna take forever!" && \
	$BUILD_DIR/rust-xtensa/configure --experimental-targets=Xtensa && \
	$BUILD_DIR/rust-xtensa/x.py build --stage 2

RUN apt-get update && apt-get install -y tree

RUN echo "Cleaning up" && \
	mv $BUILD_DIR/rust-xtensa/build $BUILD_DIR/build && \
	find $BUILD_DIR/rust-xtensa/ -maxdepth 1 -not -name 'library' | sed 1d | xargs rm -rf && \
	cd $BUILD_DIR/build && rm -rf bootstrap cache tmp && \
	cd x86_64-unknown-linux-gnu && rm -rf compiler-doc crate-docs doc md-doc stage0* stage1* && \
	cd $BUILD_DIR && tar -cf - . | xz -6 -T0 -c - > $XTENSA_RUSTC_ARCHIVE_PATH && \
	rm -rf $BUILD_DIR

RUN echo "Installing ESP32 tools..." && \
	wget https://github.com/espressif/crosstool-NG/releases/download/esp-2021r1/xtensa-esp32-elf-gcc8_4_0-esp-2021r1-linux-amd64.tar.gz -O extensa-esp32.tar.gz && \ 
	tar xvzf extensa-esp32.tar.gz && \
	rm extensa-esp32.tar.gz && \
	pip install esptool
	
ENV PATH=$PATH:$BUILD_DIR/xtensa-esp32-elf/bin
ENV XARGO_RUST_SRC=$BUILD_DIR/rust-xtensa/library
ENV RUSTC=$BUILD_DIR/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
ENV RUST_BACKTRACE=1 

WORKDIR $BUILD_DIR/test_app
RUN echo "Test demo project" && \
	mkdir -p $BUILD_DIR && cd $BUILD_DIR && tar -xf $XTENSA_RUSTC_ARCHIVE_PATH && \
	tree -L 2 $BUILD_DIR && \ 
	git clone https://github.com/MabezDev/xtensa-rust-quickstart.git && \
	cd xtensa-rust-quickstart && \
	git reset --hard cafc61a544e881960335365677911fe9ac9169ae && \
	cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" && \
	rm -rf $BUILD_DIR && \
	echo "End Test demo project"
WORKDIR /app