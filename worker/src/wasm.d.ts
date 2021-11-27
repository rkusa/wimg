declare module "*.wasm" {
  const value: string;
  export default WebAssembly.Module;
}
