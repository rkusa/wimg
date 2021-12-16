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
    const resized = resize(wimg, decoded, 128, 64);
    decoded.dealloc();
    {
      const encoded = encode(wimg, resized, "jpeg");
      await fsp.writeFile(
        "result_from_jpeg.jpg",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    {
      const encoded = encode(wimg, resized, "png");
      await fsp.writeFile(
        "result_from_jpeg.png",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    {
      const encoded = encode(wimg, resized, "avif");
      await fsp.writeFile(
        "result_from_jpeg.avif",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    {
      const encoded = encode(wimg, resized, "webp");
      await fsp.writeFile(
        "result_from_jpeg.webp",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    resized.dealloc();
  }

  {
    const image = await fsp.readFile("./example.png");
    const decoded = decode(wimg, image, "png");
    console.log("hash:", hash(wimg, decoded));
    const resized = resize(wimg, decoded, 128, 64);
    decoded.dealloc();
    {
      const encoded = encode(wimg, resized, "jpeg");
      await fsp.writeFile(
        "result_from_png.jpg",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    {
      const encoded = encode(wimg, resized, "png");
      await fsp.writeFile(
        "result_from_png.png",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    {
      const encoded = encode(wimg, resized, "avif");
      await fsp.writeFile(
        "result_from_png.avif",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    {
      const encoded = encode(wimg, resized, "webp");
      await fsp.writeFile(
        "result_from_png.webp",
        Buffer.from(encoded.asUint8Array())
      );
      encoded.dealloc();
    }
    resized.dealloc();
  }
}

run().catch(console.error);
