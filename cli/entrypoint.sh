#!/usr/bin/env sh
set -e

ARGS="--manifest out/manifest.json -b products -f jpg -f webp -f avif \
      --jpeg-quality 95 --webp-quality 95 --avif-quality 80"
valgrind --leak-check=full wimg-cli --variant big -o out -w 1200 -h 800 $ARGS products/0.*
