import { promises as fsp } from "fs";
import WASI from "./wasi";
import { webcrypto } from "crypto";

// @ts-expect-error
global.crypto = webcrypto;

async function decode(): Promise<ArrayPtr> {
  const image = await fsp.readFile("./example.jpg");

  const jpeg_decode = (await WASI.instantiate<JpegDecode>(
    fsp.readFile("../target/wasm32-wasi/release/wimg_jpeg_decode.wasm")
  )) as JpegDecode;

  // allocate memory for input image
  const inPtr = jpeg_decode.alloc(image.byteLength);
  console.log("in:", inPtr, image.byteLength);

  // write input image into memory
  new Uint8ClampedArray(jpeg_decode.memory.buffer, inPtr, image.byteLength).set(
    image
  );

  // decode and dealloc input image
  const outPtr = jpeg_decode.decode(inPtr, image.byteLength);
  jpeg_decode.dealloc(inPtr, image.byteLength);

  // read memory location of decoded image from memory
  const [resultPtr, resultLen, resultCap] = new Uint32Array(
    jpeg_decode.memory.buffer,
    outPtr,
    3
  );
  // console.log("out", resultPtr, resultLen, resultCap);

  return new ArrayPtr(jpeg_decode, outPtr);
}

async function resize(ptr: ArrayPtr) {
  const resize = await WASI.instantiate<Resize>(
    fsp.readFile("../target/wasm32-wasi/release/wimg_resize.wasm")
  );

  const input = ptr.asUint8Array();

  // allocate memory for input image
  const inPtr = resize.alloc(input.byteLength);
  console.log("in:", inPtr, input.byteLength);

  // write input image into memory
  new Uint8ClampedArray(resize.memory.buffer, inPtr, input.byteLength).set(
    input
  );

  // resize image
  const outPtr = resize.resize(inPtr, input.byteLength, 1024, 1024, 128, 128);
  // deallocate result
  ptr.dealloc();

  // read memory location of decoded image from memory
  const [resizedPtr, resizedLen, resizedCap] = new Uint32Array(
    resize.memory.buffer,
    outPtr,
    3
  );
  console.log("resized", resizedPtr, resizedLen, resizedCap);

  return new ArrayPtr(resize, outPtr);
}

async function encode(ptr: ArrayPtr) {
  const jpeg_encode = await WASI.instantiate<JpegEncode>(
    fsp.readFile("../target/wasm32-wasi/release/wimg_jpeg_encode.wasm")
  );

  const input = ptr.asUint8Array();

  // allocate memory for input image
  const inPtr = jpeg_encode.alloc(input.byteLength);
  console.log("in:", inPtr, input.byteLength);

  // write input image into memory
  new Uint8ClampedArray(jpeg_encode.memory.buffer, inPtr, input.byteLength).set(
    input
  );
  ptr.dealloc();

  // encode image
  const outPtr = jpeg_encode.encode(inPtr, input.byteLength, 128, 128);

  // read memory location of decoded image from memory
  const [encodedPtr, encodedLen, encodedCap] = new Uint32Array(
    jpeg_encode.memory.buffer,
    outPtr,
    3
  );
  console.log("encoded", encodedPtr, encodedLen, encodedCap);

  // write and deallocate encoded image
  await fsp.writeFile(
    "result.jpg",
    Buffer.from(jpeg_encode.memory.buffer, encodedPtr, encodedLen)
  );
  jpeg_encode.dealloc_vec(outPtr);
}

async function run() {
  await encode(await resize(await decode()));
}

class ArrayPtr {
  private readonly module: WimgCommon;
  private readonly ptr: number;

  public constructor(module: WimgCommon, ptr: number) {
    this.module = module;
    this.ptr = ptr;
  }

  public asUint8Array(): Uint8Array {
    const [offset, length] = new Uint32Array(
      this.module.memory.buffer,
      this.ptr,
      2
    );

    return new Uint8Array(this.module.memory.buffer, offset, length);
  }

  public dealloc() {
    this.module.dealloc_vec(this.ptr);
  }
}

run().catch(console.error);

interface WimgCommon {
  readonly memory: WebAssembly.Memory;
  alloc(length: number): number;
  dealloc(offset: number, length: number): number;
  dealloc_vec(offset: number): number;
}

interface JpegDecode extends WimgCommon {
  decode(offset: number, length: number): number;
}

interface JpegEncode extends WimgCommon {
  encode(offset: number, length: number, width: number, height: number): number;
}

interface Resize extends WimgCommon {
  resize(
    offset: number,
    length: number,
    w1: number,
    h1: number,
    w2: number,
    h2: number
  ): number;
}
