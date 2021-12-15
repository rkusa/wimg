const ERRNO_SUCCESS = 0;
const ERRNO_BADF = 8;

export default class WASI {
  private memory?: WebAssembly.Memory;

  private constructor() {}

  public static async instantiate<T>(
    bytes: BufferSource | PromiseLike<BufferSource> | WebAssembly.Module
  ): Promise<T> {
    const b = await bytes;
    let instance: WebAssembly.Instance;
    const wasi = new WASI();
    if (b instanceof WebAssembly.Module) {
      instance = await WebAssembly.instantiate(b, {
        wasi_snapshot_preview1: wasi.imports(),
      });
    } else {
      const module = await WebAssembly.instantiate(b, {
        wasi_snapshot_preview1: wasi.imports(),
      });
      instance = module.instance;
    }

    const exports = instance.exports;
    if (!exports.memory || !(exports.memory instanceof WebAssembly.Memory)) {
      throw new TypeError("expected .memory export");
    }
    wasi.setMemory(exports.memory);
    return exports as unknown as T;
  }

  public setMemory(memory: WebAssembly.Memory) {
    this.memory = memory;
  }

  private environ_get() {
    throw new Error("environ_get not implemented");
  }

  private environ_sizes_get(
    environcOffset: number,
    _environBufferSizeOffset: number
  ) {
    if (!this.memory) {
      throw new Error("memory not set");
    }

    const memoryView = new DataView(this.memory.buffer);
    memoryView.setUint32(environcOffset, 0, true);
    return ERRNO_SUCCESS;
  }

  private fd_close() {
    throw new Error("fd_close not implemented");
  }

  private fd_seek() {
    throw new Error("fd_seek not implemented");
  }

  private fd_write(
    fd: number,
    iovsOffset: number,
    iovsLength: number,
    nwrittenOffset: number
  ) {
    if (!this.memory) {
      throw new Error("memory not set");
    }

    if (fd !== 1 && fd !== 2) {
      return ERRNO_BADF;
    }

    const decoder = new TextDecoder();
    const memoryView = new DataView(this.memory.buffer);
    let nwritten = 0;
    for (let i = 0; i < iovsLength; i++) {
      const dataOffset = memoryView.getUint32(iovsOffset, true);
      iovsOffset += 4;

      const dataLength = memoryView.getUint32(iovsOffset, true);
      iovsOffset += 4;

      const data = new Uint8Array(this.memory.buffer, dataOffset, dataLength);
      const s = decoder.decode(data);
      nwritten += data.byteLength;
      switch (fd) {
        case 1: // stdout
          console.log(s);
          break;
        case 2: // stderr
          console.error(s);
          break;
        default:
          return ERRNO_BADF;
      }
    }

    memoryView.setUint32(nwrittenOffset, nwritten, true);

    return ERRNO_SUCCESS;
  }

  private proc_exit(rval: number) {
    throw new Error(`WASM program exited with code: ${rval}`);
  }

  private random_get(offset: number, length: number) {
    if (!this.memory) {
      throw new Error("memory not set");
    }

    const buffer = new Uint8Array(this.memory.buffer, offset, length);
    crypto.getRandomValues(buffer);

    return ERRNO_SUCCESS;
  }

  private sched_yield() {
    throw new Error("sched_yield not implemented");
  }

  public imports() {
    return {
      environ_get: this.environ_get.bind(this),
      environ_sizes_get: this.environ_sizes_get.bind(this),
      fd_close: this.fd_close.bind(this),
      fd_seek: this.fd_seek.bind(this),
      fd_write: this.fd_write.bind(this),
      proc_exit: this.proc_exit.bind(this),
      random_get: this.random_get.bind(this),
      sched_yield: this.sched_yield.bind(this),
    };
  }
}
