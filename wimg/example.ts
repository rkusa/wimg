import { promises as fsp } from "fs";
import WASI from "../wasi/wasi";
import { webcrypto } from "crypto";
import { decode, encode, resize, WImg } from "./wimg";

// @ts-expect-error
global.crypto = webcrypto;

async function run() {
  const wimg = await WASI.instantiate<WImg>(
    fsp.readFile("../target/wasm32-wasi/release/wimg.wasm")
  );

  const image = await fsp.readFile("./example.jpg");

  const [decoded, width, height] = await decode(wimg, image);
  console.log("dimension", width, height);
  const encoded = await encode(
    wimg,
    await resize(wimg, decoded, width, height, 128, 128),
    128,
    128
  );

  // write and deallocate encoded image
  await fsp.writeFile("result.jpg", Buffer.from(encoded.asUint8Array()));
  encoded.dealloc();
}

run().catch(console.error);