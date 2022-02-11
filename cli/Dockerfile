FROM rust:1-slim

RUN apt-get update && apt-get install -y \
    valgrind \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/wimg-cli
ADD . .
RUN --mount=type=cache,uid=1000,gid=1000,target=/usr/local/cargo/git \
    --mount=type=cache,uid=1000,gid=1000,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,uid=1000,gid=1000,target=/usr/src/wimg-cli/target \
    cargo build && cp /usr/src/wimg-cli/target/debug/wimg-cli /usr/bin/
ADD products products

ADD entrypoint.sh .
ENTRYPOINT ["./entrypoint.sh"]
