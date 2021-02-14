# This is a working enviroment for wasm and Webpack dev

## Wasm

To compile rust to wasm run in "image-to-ascii-rukarangi/":
`
wasm-pack build
`
This will compile into "pkg/" where it can be used in "www/"

## Webpack

To run the dev server:
`
npm start
`
Will compile and run a dev server.

## Note:

Will require:
* node
* rustc/cargo/etc.
* wasm-pack