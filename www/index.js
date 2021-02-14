import * as wasm from "../image-to-ascii-rukarangi/pkg";

wasm.greet()

let data = new Uint8Array([21,31])
wasm.toAscii(data)
