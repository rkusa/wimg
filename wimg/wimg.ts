export function decode(
  wimg: WImg,
  ctx: number,
  image: ArrayBuffer,
  format: "jpeg" | "png"
): Image {
  // allocate memory for input image
  const inData = wimg.alloc(image.byteLength);
  console.log("in:", inData, image.byteLength);

  // write input image into memory
  new Uint8ClampedArray(wimg.memory.buffer, inData, image.byteLength).set(
    new Uint8Array(image)
  );

  // allocate output image
  const outImg = wimg.image_new();

  // decode and dealloc input image
  const errorCode = wimg[`${format}_decode`](
    ctx,
    inData,
    image.byteLength,
    outImg
  );
  wimg.dealloc(inData, image.byteLength);
  if (errorCode < 0) {
    wimg.image_drop(outImg);
    throwLastError(wimg, ctx, errorCode);
  }

  return new Image(wimg, outImg);
}

export function resize(
  wimg: WImg,
  ctx: number,
  img: Image,
  newWidth: number,
  newHeight: number,
  maintainAspect: boolean
) {
  // allocate output image
  const outImg = wimg.image_new();

  // resize image
  const errorCode = wimg.resize(
    ctx,
    img.ptr,
    newWidth,
    newHeight,
    maintainAspect,
    outImg
  );
  if (errorCode < 0) {
    wimg.image_drop(outImg);
    throwLastError(wimg, ctx, errorCode);
  }

  return new Image(wimg, outImg);
}

export function encode(
  wimg: WImg,
  ctx: number,
  img: Image,
  format: "jpeg" | "png" | "avif" | "webp"
): Image {
  // allocate output image
  const outImg = wimg.image_new();

  // encode image
  const errorCode = wimg[`${format}_encode`](ctx, img.ptr, outImg);
  if (errorCode < 0) {
    wimg.image_drop(outImg);
    throwLastError(wimg, ctx, errorCode);
  }

  return new Image(wimg, outImg);
}

function throwLastError(wimg: WImg, ctx: number, code: number): void {
  const m = wimg.last_error_message(ctx);
  if (m) {
    const decoder = new TextDecoder();

    let memory = new Uint8Array(wimg.memory.buffer);
    let len = 0;
    for (; memory[len + m] != 0; len++) {}

    const data = new Uint8Array(wimg.memory.buffer, m, len);
    const err = decoder.decode(data);
    wimg.error_message_drop(m);
    throw new Error(err);
  } else {
    throw new Error(`error code ${code}`);
  }
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

  public drop() {
    this.module.image_drop(this.ptr);
  }
}

export type ErrorCode = number;
export type Ptr = number;
export type ContextPtr = Ptr;
export type StringPtr = Ptr;
export type ImagePtr = Ptr;

export interface WImg {
  readonly memory: WebAssembly.Memory;

  context_new(): ContextPtr;
  context_drop(ctx: ContextPtr): void;

  last_error_message(ctx: ContextPtr): StringPtr;
  error_message_drop(error: StringPtr): void;

  alloc(length: number): Ptr;
  dealloc(ptr: Ptr, length: number): void;

  image_new(): ImagePtr;
  image_drop(ptr: ImagePtr): void;

  resize(
    ctx: ContextPtr,
    img: ImagePtr,
    newWidth: number,
    newHeight: number,
    maintainAspect: boolean,
    out: ImagePtr
  ): ErrorCode;

  jpeg_decode(
    ctx: ContextPtr,
    ptr: Ptr,
    length: number,
    out: ImagePtr
  ): ErrorCode;
  jpeg_encode(ctx: ContextPtr, img: ImagePtr, out: ImagePtr): ErrorCode;

  png_decode(
    ctx: ContextPtr,
    ptr: Ptr,
    length: number,
    out: ImagePtr
  ): ErrorCode;
  png_encode(ctx: ContextPtr, img: ImagePtr, out: ImagePtr): ErrorCode;

  avif_encode(ctx: ContextPtr, img: ImagePtr, out: ImagePtr): ErrorCode;

  webp_encode(ctx: ContextPtr, img: ImagePtr, out: ImagePtr): ErrorCode;
}
