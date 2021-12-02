export async function decode(
  wimg: WImg,
  image: ArrayBuffer
): Promise<[ArrayPtr, number, number]> {
  // allocate memory for input image
  const inPtr = wimg.alloc(image.byteLength);
  console.log("in:", inPtr, image.byteLength);

  // write input image into memory
  new Uint8ClampedArray(wimg.memory.buffer, inPtr, image.byteLength).set(
    new Uint8Array(image)
  );

  // decode and dealloc input image
  const outPtr = wimg.decode_jpeg(inPtr, image.byteLength);
  wimg.dealloc(inPtr, image.byteLength);

  // read memory location of decoded image from memory
  const [resultPtr, resultLen] = new Uint32Array(wimg.memory.buffer, outPtr, 3);
  // console.log("out", resultPtr, resultLen, resultCap);

  const dv = new DataView(wimg.memory.buffer, resultPtr, 8);
  const width = dv.getUint32(0, false);
  const height = dv.getUint32(4, false);

  return [new ArrayPtr(wimg, outPtr, 8), width, height];
}

export async function resize(
  wimg: WImg,
  ptr: ArrayPtr,
  w1: number,
  h1: number,
  w2: number,
  h2: number
) {
  const [offset, length] = ptr.offsetLength();

  // resize image
  const outPtr = wimg.resize(offset, length, w1, h1, w2, h2);
  // deallocate result
  ptr.dealloc();

  return new ArrayPtr(wimg, outPtr);
}

export async function encode(
  wimg: WImg,
  ptr: ArrayPtr,
  width: number,
  height: number
): Promise<ArrayPtr> {
  const [offset, length] = ptr.offsetLength();

  // encode image
  const outPtr = wimg.encode_jpeg(offset, length, width, height);
  ptr.dealloc();

  return new ArrayPtr(wimg, outPtr);
}

export class ArrayPtr {
  private readonly module: WImg;
  public readonly ptr: number;
  private readonly offset?: number;

  public constructor(module: WImg, ptr: number, offset?: number) {
    this.module = module;
    this.ptr = ptr;
    this.offset = offset;
  }

  public offsetLength(): [number, number] {
    const [offset, length] = new Uint32Array(
      this.module.memory.buffer,
      this.ptr,
      2
    );
    return [offset, length];
  }

  public asUint8Array(): Uint8Array {
    const [offset, length] = this.offsetLength();

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

export interface WImg {
  readonly memory: WebAssembly.Memory;
  alloc(length: number): number;
  dealloc(offset: number, length: number): number;
  dealloc_vec(offset: number): number;
  decode_jpeg(offset: number, length: number): number;
  encode_jpeg(
    offset: number,
    length: number,
    width: number,
    height: number
  ): number;
  resize(
    offset: number,
    length: number,
    w1: number,
    h1: number,
    w2: number,
    h2: number
  ): number;
}
