import * as wasm from "../image-to-ascii-rukarangi/pkg";

wasm.init_panic()
wasm.greet()

let data = new Uint8Array([21,31])
wasm.toAscii(data)

let input = document.getElementById("image-input")
let display = document.getElementById("byte-para")
let xMod = document.getElementById("x-axis")
let yMod = document.getElementById("y-axis")
let reloader = document.getElementById("reload")

let currentBuf;

function converterHandler(buf, xValue, yValue, output) {
    let result_2 = wasm.different_methods(buf,yValue,xValue)
    output.textContent = result_2
}

function readFile(input) {
    let file = input.files[0]

    let xValue = xMod.value
    let yValue = yMod.value

    let reader = new FileReader()

    reader.readAsArrayBuffer(file)

    let bytes

    reader.onload = () => {
        console.log("Loaded reader")
        bytes = reader.result
        console.log("Bytes: " + bytes)

        currentBuf = new Uint8Array(bytes)
        wasm.toAscii(currentBuf)
        converterHandler(currentBuf, xValue, yValue, display)
    }

    reader.onerror = () => {
        console.log("reader.error")
    }
}
input.addEventListener("change", (event) => {
    readFile(event.target)
})

reloader.addEventListener("click", (event) => {
    let xValue = xMod.value
    let yValue = yMod.value
    converterHandler(currentBuf, xValue, yValue, display)
})