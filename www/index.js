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
    let converter = wasm.Converter.new(buf)

    let string = new TextDecoder("utf-8").decode(buf)
    output.textContent = "Image Read"
    //console.log(string)
        
    converter.test_pattern()
    converter.populate_ihdr()
    converter.display_head()
    converter.populate_idat()
    converter.decode_idat()
        
    let result = converter.filter(yValue,xValue)
    let result_2 = converter.different_methods(yValue,xValue)
    //console.log(result)
    //result = result.replace(/(?:\r\n|\r|\n)/g, '<br>');
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