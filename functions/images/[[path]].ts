import WASI from "../../wasi/wasi";
import wimg_jpeg_decode from "../../wasm/wimg_jpeg_decode.wasm";
import wimg_jpeg_encode from "../../wasm/wimg_jpeg_encode.wasm";
import wimg_resize from "../../wasm/wimg_resize.wasm";

export async function onRequestGet(_: EventContext<{}, string, unknown>) {
  const res = await fetch(
    "https://images.pexels.com/photos/416682/pexels-photo-416682.jpeg?w=1000"
  );

  console.time("transform");

  const [ptr, imgWidth, imgHeight] = await decode(await res.arrayBuffer());
  console.log("size", imgWidth, imgHeight);

  const image = await encode(
    await resize(ptr, imgWidth, imgHeight, 500, 333),
    500,
    333
  );

  console.timeEnd("transform");

  return new Response(image, {
    headers: {
      "Content-Type": "image/jpeg",
    },
  });
}

async function decode(image: ArrayBuffer): Promise<[ArrayPtr, number, number]> {
  const jpeg_decode = (await WASI.instantiate<JpegDecode>(
    wimg_jpeg_decode
  )) as JpegDecode;

  // allocate memory for input image
  const inPtr = jpeg_decode.alloc(image.byteLength);
  console.log("in:", inPtr, image.byteLength);

  // write input image into memory
  new Uint8ClampedArray(jpeg_decode.memory.buffer, inPtr, image.byteLength).set(
    new Uint8Array(image)
  );

  // decode and dealloc input image
  const outPtr = jpeg_decode.decode(inPtr, image.byteLength);
  jpeg_decode.dealloc(inPtr, image.byteLength);

  // read memory location of decoded image from memory
  const [resultPtr] = new Uint32Array(jpeg_decode.memory.buffer, outPtr, 1);
  const dv = new DataView(jpeg_decode.memory.buffer, resultPtr, 8);
  const width = dv.getUint32(0, false);
  const height = dv.getUint32(4, false);

  return [new ArrayPtr(jpeg_decode, outPtr, 8), width, height];
}

async function resize(
  ptr: ArrayPtr,
  w1: number,
  h1: number,
  w2: number,
  h2: number
) {
  const resize = await WASI.instantiate<Resize>(wimg_resize);

  const input = ptr.asUint8Array();

  // allocate memory for input image
  const inPtr = resize.alloc(input.byteLength);
  console.log("in:", inPtr, input.byteLength);

  // write input image into memory
  new Uint8ClampedArray(resize.memory.buffer, inPtr, input.byteLength).set(
    input
  );

  // resize image
  const outPtr = resize.resize(inPtr, input.byteLength, w1, h1, w2, h2);
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

async function encode(
  ptr: ArrayPtr,
  width: number,
  height: number
): Promise<Uint8Array> {
  const jpeg_encode = await WASI.instantiate<JpegEncode>(wimg_jpeg_encode);

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
  const outPtr = jpeg_encode.encode(inPtr, input.byteLength, width, height);

  // read memory location of decoded image from memory
  const [encodedPtr, encodedLen, encodedCap] = new Uint32Array(
    jpeg_encode.memory.buffer,
    outPtr,
    3
  );
  console.log("encoded", encodedPtr, encodedLen, encodedCap);

  // write and deallocate encoded image
  return new Uint8Array(jpeg_encode.memory.buffer, encodedPtr, encodedLen);
}

class ArrayPtr {
  private readonly module: WimgCommon;
  private readonly ptr: number;
  private readonly offset?: number;

  public constructor(module: WimgCommon, ptr: number, offset?: number) {
    this.module = module;
    this.ptr = ptr;
    this.offset = offset;
  }

  public asUint8Array(): Uint8Array {
    const [offset, length] = new Uint32Array(
      this.module.memory.buffer,
      this.ptr,
      2
    );

    return new Uint8Array(
      this.module.memory.buffer,
      offset + (this.offset ?? 0),
      length
    );
  }

  public dealloc() {
    this.module.dealloc_vec(this.ptr);
  }
}

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
