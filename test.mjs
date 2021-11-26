import { promises as fsp } from 'fs'
import { WASI } from 'wasi';

const image = await fsp.readFile("./example.jpg")
const code = await fsp.readFile("./target/wasm32-wasi/release/mozjpeg.wasm")
const wasi = new WASI();

const module = await WebAssembly.instantiate(code, {
  wasi_snapshot_preview1: wasi.wasiImport
});
wasi.initialize(module.instance)

const exports = module.instance.exports

// allocate memory for input image
const inPtr = exports.alloc(image.byteLength)
console.log('in:', inPtr, image.byteLength)

// write input image into memory
new Uint8ClampedArray(exports.memory.buffer, inPtr, image.byteLength).set(image);

// decode and dealloc input image
const outPtr = exports.decode(inPtr, image.byteLength);
exports.dealloc(inPtr, image.byteLength);

// read memory location of decoded image from memory
const [resultPtr, resultLen, resultCap] = new Uint32Array(exports.memory.buffer, outPtr, 3);
console.log('out', resultPtr, resultLen, resultCap)

// read decoded image from memory
const result = new Uint8ClampedArray(exports.memory.buffer, resultPtr, resultLen)
console.log(result.length)

// deallocate result
exports.dealloc_vec(outPtr);

