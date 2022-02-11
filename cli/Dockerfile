FROM rust:1-slim

RUN apt-get update && apt-get install -y \
    valgrind \
    && rm -rf /var/lib/apt/lists/*

ADD . /usr/src/wimg
WORKDIR /usr/src/wimg
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,target=/usr/src/wimg/target \
    cargo build --release -p wimg-cli --no-default-features && cp target/release/wimg-cli /usr/bin/
ADD cli/products products

ADD cli/entrypoint.sh .
ENTRYPOINT ["./entrypoint.sh"]
