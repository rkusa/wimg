FROM rust:1
RUN apt-get update && apt-get install -qqy autoconf libtool pkg-config curl build-essential cmake

RUN rustup target add wasm32-wasi

WORKDIR /opt/wasi-sdk
RUN export WASI_VERSION=14 && \
      export WASI_VERSION_FULL=${WASI_VERSION}.0 && \
      curl -LO https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
      tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz

ENV WASI_SDK_PATH=/opt/wasi-sdk/wasi-sdk-14.0

ENV PATH="/opt/wasi-sdk/wasi-sdk-14.0/bin:${PATH}"
