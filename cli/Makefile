# Install build dependencies:
# ```
# brew install FiloSottile/musl-cross/musl-cross
# rustup target add x86_64-unknown-linux-musl
# ```
build:
	TARGET_CC=x86_64-linux-musl-gcc \
		RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" \
		cargo build --release --target x86_64-unknown-linux-musl

