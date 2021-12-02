#!/bin/sh
set -e

rm -rf ./dist/*
mkdir -p dist

./node_modules/.bin/esbuild --format=esm --platform=neutral --external:"*.wasm" --outdir=./dist --bundle src/worker.ts

cp ../target/wasm32-wasi/release/wimg.wasm dist/
