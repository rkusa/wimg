import { promises as fsp } from "fs";
import WASI from "../wasi/wasi";
import { decode, encode, resize, WImg, Image } from "./wimg";

async function run() {
  const wimg = await WASI.instantiate<WImg>(
    fsp.readFile("../target/wasm32-wasi/release/wimg.wasm")
  );

  const ctx = wimg.context_new();
  let decoded: Image | undefined = undefined;
  let resized: Image | undefined = undefined;
  let encoded: Image | undefined = undefined;

  try {
    const decodeFormats = ["jpeg", "png"] as const;
    const encodeFormats = ["jpeg", "png", "avif", "webp"] as const;
    for (const decodeFormat of decodeFormats) {
      const image = await fsp.readFile(`./example.${ext(decodeFormat)}`);
      decoded = decode(wimg, ctx, image, decodeFormat);
      resized = resize(wimg, ctx, decoded, 128, 64, false);
      decoded.drop();
      decoded = undefined;

      for (const encodeFormat of encodeFormats) {
        encoded = encode(wimg, ctx, resized, encodeFormat);
        await fsp.writeFile(
          `result_from_${decodeFormat}.${ext(encodeFormat)}`,
          Buffer.from(encoded.asUint8Array())
        );
        encoded.drop();
        encoded = undefined;
      }

      resized.drop();
      resized = undefined;
    }
  } finally {
    if (decoded) {
      decoded.drop();
    }
    if (resized) {
      resized.drop();
    }
    if (encoded) {
      encoded.drop();
    }
    wimg.context_drop(ctx);
  }
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
