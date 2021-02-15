import * as wasm from "../image-to-ascii-rukarangi/pkg";

wasm.greet()

let data = new Uint8Array([21,31])
wasm.toAscii(data)

let input = document.getElementById("image-input")
let display = document.getElementById("byte-para")

function readFile(input) {
    let file = input.files[0]

    let reader = new FileReader()

    reader.readAsArrayBuffer(file)

    let bytes

    reader.onload = () => {
        console.log("Loaded reader")
        bytes = reader.result
        console.log("Bytes: " + bytes)

        let newData = new Uint8Array(bytes)
        wasm.toAscii(newData)
        let converter = wasm.Converter.new(newData)

        let string = new TextDecoder("utf-8").decode(newData)
        display.textContent = string
        converter.display_head()
        converter.test_pattern()
        converter.populate_ihdr()

    }

    reader.onerror = () => {
        console.log("reader.error")
    }
}

input.addEventListener("change", (event) => {
    readFile(event.target)
})