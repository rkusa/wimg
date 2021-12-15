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

  {
    const image = await fsp.readFile("./example.jpg");
    const decoded = decode(wimg, image, "jpeg");
    console.log("hash:", hash(wimg, decoded));
    const encoded = encode(wimg, resize(wimg, decoded, 128, 64), "png");
    await fsp.writeFile("result.png", Buffer.from(encoded.asUint8Array()));
    encoded.dealloc();
  }

  {
    const image = await fsp.readFile("./example.png");
    const decoded = decode(wimg, image, "png");
    console.log("hash:", hash(wimg, decoded));
    const encoded = encode(wimg, resize(wimg, decoded, 128, 64), "png");
    await fsp.writeFile("result.jpg", Buffer.from(encoded.asUint8Array()));
    encoded.dealloc();
  }
}

run().catch(console.error);
