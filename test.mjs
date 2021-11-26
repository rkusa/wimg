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

// RESIZE

// resize image
const resizedVec = exports.resize(resultPtr, resultLen, 372, 421, 161, 210);
// deallocate result
exports.dealloc_vec(outPtr);

// read memory location of decoded image from memory
const [resizedPtr, resizedLen, resizedCap] = new Uint32Array(exports.memory.buffer, resizedVec, 3);
console.log('resized', resizedPtr, resizedLen, resizedCap)


//
// ENCODE
//

// encode image
const encodedVec = exports.encode(resizedPtr, resizedLen, 161, 210);
// deallocate resized
exports.dealloc_vec(resizedVec);

// read memory location of decoded image from memory
const [encodedPtr, encodedLen, encodedCap] = new Uint32Array(exports.memory.buffer, encodedVec, 3);
console.log('encoded', encodedPtr, encodedLen, encodedCap)

// read decoded image from memory
const encoded = new Uint8ClampedArray(exports.memory.buffer, encodedPtr, encodedLen)
console.log(encoded.length)

// write and deallocate encoded image
await fsp.writeFile("result.jpg", Buffer.from(exports.memory.buffer, encodedPtr, encodedLen))
exports.dealloc_vec(encodedVec);
