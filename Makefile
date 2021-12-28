.PHONY: build
build:
	PATH="$(shell pwd)/wasi-sdk/dist/wasi-sdk-14.0/bin:${PATH}" \
	CFLAGS="--sysroot=$(shell pwd)/wasi-sdk/dist/wasi-sdk-14.0/share/wasi-sysroot" \
		cargo build --release --target wasm32-wasi

.PHONY: build_docker
build_docker:
	docker build --platform linux/amd64 -t mozjpeg-builder . --progress=plain
	docker run --rm --platform linux/amd64 --user "$(shell id -u)":"$(shell id -g)" \
		-v $(shell pwd):/usr/src/mozjpeg -w /usr/src/mozjpeg mozjpeg-builder cargo build --release

build_linux_musl:
	TARGET_CC=x86_64-linux-musl-gcc \
	RUSTFLAGS="-C linker=x86_64-linux-musl-gcc -C target-feature=-crt-static" \
		cargo build --release --target x86_64-unknown-linux-musl --features ffi

.PHONY: test
test:
	cd wimg && npm run build && node example.mjs
