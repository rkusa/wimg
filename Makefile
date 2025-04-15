.PHONY: build
build:
	PATH="$(shell pwd)/wasi-sdk/dist/wasi-sdk-16.0/bin:${PATH}" \
	CFLAGS="--sysroot=$(shell pwd)/wasi-sdk/dist/wasi-sdk-16.0/share/wasi-sysroot" \
		cargo build --release --target wasm32-wasi --features ffi

.PHONY: build_docker
build_docker:
	docker build --platform linux/amd64 -t mozjpeg-builder . --progress=plain
	docker run --rm --platform linux/amd64 --user "$(shell id -u)":"$(shell id -g)" \
		-v $(shell pwd):/usr/src/mozjpeg -w /usr/src/mozjpeg mozjpeg-builder cargo build --release

build_linux: build_x86_64_linux_musl build_aarch64_linux_musl

build_x86_64_linux_musl:
	cargo build --release --target x86_64-unknown-linux-musl --features ffi

build_aarch64_linux_musl:
	cargo build --release --target aarch64-unknown-linux-musl --features ffi

.PHONY: test
test:
	cd wimg && pnpm run build && node example.mjs

leak_test:
	docker build --platform linux/arm64 -f cli/Dockerfile -t wimg-cli . --progress=plain
	docker run --rm --platform linux/arm64 -t wimg-cli
