export async function decode(
  wimg: WImg,
  image: ArrayBuffer
): Promise<[Image, number, number]> {
  // allocate memory for input image
  const inPtr = wimg.alloc(image.byteLength);
  console.log("in:", inPtr, image.byteLength);

  // write input image into memory
  new Uint8ClampedArray(wimg.memory.buffer, inPtr, image.byteLength).set(
    new Uint8Array(image)
  );

  // decode and dealloc input image
  const outPtr = wimg.jpeg_decode(inPtr, image.byteLength);
  wimg.dealloc(inPtr, image.byteLength);

  // read memory location of decoded image from memory
  const [_ptr, _len, _cap, width, height] = new Uint32Array(
    wimg.memory.buffer,
    outPtr,
    3
  );

  return [new Image(wimg, outPtr), width, height];
}

export async function resize(
  wimg: WImg,
  img: Image,
  newWidth: number,
  newHeight: number
) {
  // resize image
  const outPtr = wimg.resize(img.ptr, newWidth, newHeight);
  // deallocate result
  img.dealloc();

  return new Image(wimg, outPtr);
}

export async function encode(wimg: WImg, img: Image): Promise<Image> {
  // encode image
  const outPtr = wimg.jpeg_encode(img.ptr);
  img.dealloc();

  return new Image(wimg, outPtr);
}

export class Image {
  private readonly module: WImg;
  public readonly ptr: number;

  public constructor(module: WImg, ptr: number) {
    this.module = module;
    this.ptr = ptr;
  }

  private offsetLength(): [number, number] {
    const [offset, length] = new Uint32Array(
      this.module.memory.buffer,
      this.ptr,
      2
    );
    return [offset, length];
  }

  public asUint8Array(): Uint8Array {
    const [offset, length] = this.offsetLength();

    return new Uint8Array(this.module.memory.buffer, offset, length);
  }

  public dealloc() {
    this.module.image_destroy(this.ptr);
  }
}

export interface WImg {
  readonly memory: WebAssembly.Memory;
  alloc(length: number): number;
  dealloc(offset: number, length: number): number;
  image_destroy(offset: number): number;
  jpeg_decode(offset: number, length: number): number;
  jpeg_encode(offset: number): number;
  resize(offset: number, newWidth: number, newHeight: number): number;
}
