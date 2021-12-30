export function decode(
  wimg: WImg,
  ctx: number,
  image: ArrayBuffer,
  format: "jpeg" | "png"
): Image {
  // allocate memory for input image
  const inPtr = wimg.alloc(image.byteLength);
  console.log("in:", inPtr, image.byteLength);

  // write input image into memory
  new Uint8ClampedArray(wimg.memory.buffer, inPtr, image.byteLength).set(
    new Uint8Array(image)
  );

  // decode and dealloc input image
  const outPtr = checkError(
    wimg,
    ctx,
    wimg[`${format}_decode`](ctx, inPtr, image.byteLength)
  );
  wimg.dealloc(inPtr, image.byteLength);

  return new Image(wimg, outPtr);
}

export function resize(
  wimg: WImg,
  ctx: number,
  img: Image,
  newWidth: number,
  newHeight: number
) {
  // resize image
  const outPtr = checkError(
    wimg,
    ctx,
    wimg.resize(ctx, img.ptr, newWidth, newHeight)
  );
  return new Image(wimg, outPtr);
}

export function encode(
  wimg: WImg,
  ctx: number,
  img: Image,
  format: "jpeg" | "png" | "avif" | "webp"
): Image {
  // encode image
  const outPtr = checkError(wimg, ctx, wimg[`${format}_encode`](ctx, img.ptr));
  return new Image(wimg, outPtr);
}

export function hash(wimg: WImg, img: Image): string {
  const out = wimg.alloc(8);
  wimg.hash(img.ptr, out);
  const data = new Uint8Array(wimg.memory.buffer, out, 8);
  const hex = data.reduce((hex, b) => {
    hex += b.toString(16);
    return hex;
  }, "");

  wimg.dealloc(out, 32);

  return hex;
}

function checkError(wimg: WImg, ctx: number, img: number): number {
  if (!img) {
    const m = wimg.last_error_message(ctx);
    if (m) {
      const decoder = new TextDecoder();

      let memory = new Uint8Array(wimg.memory.buffer);
      let len = 0;
      for (; memory[len + m] != 0; len++) {}

      const data = new Uint8Array(wimg.memory.buffer, m, len);
      const err = decoder.decode(data);
      wimg.error_message_destroy(m);
      throw new Error(err);
    } else {
      throw new Error("unknown error");
    }
  }
  return img;
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
  context_new(): number;
  context_destroy(ctx: number): void;
  alloc(length: number): number;
  dealloc(offset: number, length: number): number;
  image_destroy(offset: number): number;
  jpeg_decode(ctx: number, offset: number, length: number): number;
  jpeg_encode(ctx: number, offset: number): number;
  png_decode(ctx: number, offset: number, length: number): number;
  png_encode(ctx: number, offset: number): number;
  avif_encode(ctx: number, offset: number): number;
  webp_encode(ctx: number, offset: number): number;
  heif_decode(ctx: number, offset: number, length: number): number;
  resize(
    ctx: number,
    offset: number,
    newWidth: number,
    newHeight: number
  ): number;
  last_error_message(ctx: number): number;
  error_message_destroy(offset: number): void;
  hash(image: number, out: number): void;
}
