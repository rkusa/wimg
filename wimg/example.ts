import { promises as fsp } from "fs";
import WASI from "../wasi/wasi";
import { webcrypto } from "crypto";
import { decode, encode, hash, resize, WImg } from "./wimg";

// @ts-expect-error
global.crypto = webcrypto;

async function run() {
  const wimg = await WASI.instantiate<WImg>(
    fsp.readFile("../target/wasm32-wasi/release/wimg.wasm")
  );

  const ctx = wimg.context_new();

  const decodeFormats = ["jpeg", "png"] as const;
  const encodeFormats = ["jpeg", "png", "avif", "webp"] as const;
  for (const decodeFormat of decodeFormats) {
    const image = await fsp.readFile(`./example.${ext(decodeFormat)}`);
    const decoded = decode(wimg, ctx, image, decodeFormat);
    console.log("hash:", hash(wimg, decoded));
    const resized = resize(wimg, ctx, decoded, 128, 64);
    decoded.dealloc();

    for (const encodeFormat of encodeFormats) {
      const encoded = encode(wimg, ctx, resized, encodeFormat);
      await fsp.writeFile(
        `result_from_${decodeFormat}.${ext(encodeFormat)}`,
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }

    resized.dealloc();
  }

  wimg.context_destroy(ctx);
}

function ext(format: "jpeg" | "png" | "avif" | "webp") {
  switch (format) {
    case "jpeg":
      return "jpg";
    default:
      return format;
  }
}

run().catch(console.error);
