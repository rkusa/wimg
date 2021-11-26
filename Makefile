.PHONY: build
build:
	PATH="$(shell pwd)/wasi-sdk-14.0/bin:${PATH}" cargo build --release

.PHONY: build_docker
build_docker:
	docker build --platform linux/amd64 -t mozjpeg-builder . --progress=plain
	docker run --rm --platform linux/amd64 --user "$(shell id -u)":"$(shell id -g)" \
		-v $(shell pwd):/usr/src/mozjpeg -w /usr/src/mozjpeg mozjpeg-builder cargo build --release

.PHONY: test
test:
	node --experimental-wasi-unstable-preview1 test.mjs
