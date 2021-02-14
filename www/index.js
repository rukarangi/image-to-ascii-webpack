import * as wasm from "../image-to-ascii-rukarangi/pkg";

wasm.greet()

let data = new Uint8Array([21,31])
wasm.toAscii(data)

let input = document.getElementById("image-input")

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

    }

    reader.onerror = () => {
        console.log("reader.error")
    }
}

input.addEventListener("change", (event) => {
    readFile(event.target)
})