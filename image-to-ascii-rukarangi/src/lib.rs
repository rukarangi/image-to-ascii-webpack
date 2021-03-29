extern crate console_error_panic_hook;

mod utils;
mod parser;
mod handler;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use wasm_bindgen::prelude::*;
use inflate::inflate_bytes_zlib;
use std::io::Write;
use std::str::from_utf8;
use std::panic;
use itertools::concat;
use image::codecs::png;



//init_panic();

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub fn set_panic_hook() {
//     // When the `console_error_panic_hook` feature is enabled, we can call the
//     // `set_panic_hook` function at least once during initialization, and then
//     // we will get better error messages if our code ever panics.
//     //
//     // For more details see
//     // https://github.com/rustwasm/console_error_panic_hook#readme
// #[cfg(feature = "console_error_panic_hook")]
// console_error_panic_hook::set_once!();
// }

//set_panic_hook();

#[wasm_bindgen]
extern {
    //fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_panic() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello, image-to-ascii!");
}

#[wasm_bindgen(js_name = toAscii)]
pub fn to_ascii(data_raw: Vec<u8>) {
    if data_raw.len() > 20 {
        log("function being used!");
        log_list(data_raw, 16);
    }
}

fn log_list(data_raw: Vec<u8>, length: usize) {
    for i in 0..length{
        log(&format!("{:X?}", data_raw[i])[..]);
    }
}

#[wasm_bindgen]
pub struct Converter {
    result: String,
    data_raw: Vec<u8>,
    data_decoded: Vec<u8>,
    png: parser::PngImage,
    original: Vec<u8>,
}

#[wasm_bindgen]
impl Converter {
    pub fn different_methods(&self, y_modifier: u32, x_modifier: u32) -> String {
        let result = handler::handle_new(&self.original, y_modifier, x_modifier);

        return result;
    }

    pub fn filter(&mut self, y_modifier: u32, x_modifier: u32) -> String {

        let pixel_type = handler::Pixel_type::Rgba;
        let width = as_u32_be(&self.png.ihdr.width);
        let height = as_u32_be(&self.png.ihdr.height);


        let result = handler::handle(self.data_decoded.clone(), pixel_type, y_modifier, x_modifier, width, height);
        return result;
    }

    pub fn new(data_raw: Vec<u8>) -> Converter {
        let original = data_raw.clone();
        let result = String::from("");
        let png = parser::PngImage::new_empty();

        let data_decoded = Vec::<u8>::new();

        let mut first_eight: [u8; 8] = [0,0,0,0,0,0,0,0];
        for i in 0..8 {
            first_eight[i] = data_raw[i];
        }
        
        
        if first_eight == parser::PNG {
            log("file is png");
        } else {
            log("file is missing png header");
        }
        return Converter {
            result,
            data_raw,
            data_decoded,
            png: png,
            original,
        };
    }

    pub fn populate_ihdr(&mut self) {
        let index = self.find_pattern(0 as usize, parser::IHDR.to_vec());
        let mut ihdr_bytes: [u8; 13] = [0; 13];
        for i in 0..13 {
            ihdr_bytes[i] = self.data_raw[i+index];
        }
        log(&format!("{:X?}", ihdr_bytes)[..]);
        let ihdr = parser::IhdrChunk::build(ihdr_bytes);
        self.png.ihdr = ihdr;
    }

    pub fn find_idats(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        let pattern = parser::IDAT.to_vec();

        for (idx, byte) in self.data_raw.iter().enumerate() {
            if idx + 4 >= self.data_raw.len() {
                break;
            }

            let mut four: [u8; 4] = [0; 4];
            for i in 0..4 {
                four[i] = self.data_raw[i + idx];
            }
    
            if four.to_vec() == pattern {
                log(&format!("{}", idx));
                result.push(idx + 4);
            }
        }
        log(&format!("Idat indexs {:X?}", result)[..]);
        return result;
    }

    pub fn get_data(&self, idx: usize) -> Vec<u8> {
        let length_bytes = &self.data_raw[(idx-8)..(idx-4)];
        let length: usize = as_u32_be(length_bytes) as usize;
        log(&format!("Length Bytes {:X?}", length_bytes)[..]);
        log(&format!("length {:?}", length)[..]);

        let data: Vec<u8> = (&self.data_raw[(idx)..(idx+length)]).to_vec();
        log(&format!("data length {:X?}", data.len())[..]);
        return data;
    }

    pub fn populate_idat(&mut self) {
        
        let idats: Vec<usize> = self.find_idats();
        let dats: Vec<Vec<u8>> = idats.iter().map(|x| self.get_data(*x)).collect();
        let dat: Vec<u8> = concat(dats);

        log(&format!("Number of Idats: {:?}", idats.len()));
        log(&format!("bytes {:X?}", dat)[..]);
        self.data_raw = dat;
    }

    pub fn decode_idat(&mut self) {
        let data_1 = self.data_raw.clone();
        let data = &data_1[2..]; 
        // THIS is a weird zlib peculiarity it does not like the first two bytes
        let mut decoded = Vec::<u8>::new();

        let mut decoder = inflate::InflateWriter::new(Vec::new());
        match decoder.write(&data) {
            Ok(x) => log("decoder decoded"),
            Err(e) => {
                log(&format!("decoder failed {}", e)[..]);
                log(&format!("bytes {:X?}", data)[..]);
            }
        }
        match decoder.finish() {
            Ok(x) => decoded = x,
            Err(_) => decoded = vec![0x49]
        }

        log("Decoded Idat");
        //log(&format!("bytes: {:X?}", &decoded)[..]);
        

        self.data_decoded = decoded.clone();

        log(&format!("bytes: {:X?}", &self.data_decoded)[..]);
    }

    pub fn display_head(&self) {
        log(&format!("Head Information:", )[..]);
        log(&format!("width: {:X?}", as_u32_be(&self.png.ihdr.width))[..]);
        log(&format!("height: {:X?}", as_u32_be(&self.png.ihdr.height))[..]);
        log(&format!("depth: {:X?}", self.png.ihdr.depth)[..]);
        log(&format!("color_type: {:X?}", self.png.ihdr.color_type)[..]);
        log(&format!("compression: {:X?}", self.png.ihdr.compression)[..]);
        log(&format!("filter: {:X?}", self.png.ihdr.filter)[..]);
        log(&format!("interlaced: {:X?}", self.png.ihdr.interlaced)[..]);
        log(&format!("total byte length: {:X?}", self.data_raw.len()));
    }

    pub fn test_pattern(&self) {
        self.find_pattern(0 as usize, parser::IHDR.to_vec());
    }

    pub fn find_pattern(&self, offest: usize, pattern: Vec<u8>) -> usize {
        let mut index: usize = 0; // default will be seen as not found

        for (idx, byte) in self.data_raw.iter().enumerate() {
            if idx < offest || (idx + 4) >= self.data_raw.len() {
                continue;
            }

            let mut four: [u8; 4] = [0; 4];
            for i in 0..4 {
                four[i] = self.data_raw[i + idx];
            }

            if four.to_vec() == pattern {
                index = idx + 4;
                log(&format!("{}", index));
                break;
            }
        }
        
        //log(&format!("{}", index)[..]);
        return index; // will be zero if not found
    }
}

fn as_u32_be(slice: &[u8]) -> u32 {
    let array: [u8; 4] = [slice[0],slice[1],slice[2],slice[3]];
    log("converting slice to array");
    return ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0);
}

fn as_u32_le(slice: &[u8]) -> u32 {
    let array: [u8; 4] = [slice[0],slice[1],slice[2],slice[3]];

    return ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24);
}

/*

fn main() {
    // --- Take Arguments ---

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let target_path = Path::new(&args[1]);
    let out_path = Path::new(&args[2]);
    let x_modifier = &args[3].parse::<u32>().unwrap();
    let y_modifier = &args[4].parse::<u32>().unwrap();
    let filter_input: &str = &args[5][..];
    let reverse_input = &args[6].parse::<i32>().unwrap();

    let reverse: bool;
    if *reverse_input == 1 {
        reverse = true;
    } else {
        reverse = false;
    }

    // --- Create Access to Output file ---

    // create path
    let path = Path::new(out_path);
    let display = path.display();

    // open file in write-only
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // --- Open Image and get byte Vec ---

    let img = match image::open(Path::new(target_path)) {
        Err(why) => panic!("Couldn't read image {}: {}", target_path.display(), why),
        Ok(img) => img,
    };

    println!("Dimensions: {:?}", img.dimensions());
    println!("Expected output dimensions (Rough): {} {}", img.dimensions().0 / x_modifier, img.dimensions().1 / y_modifier);

    let mut result_before = String::from("");

    let mut _count = 0;
    for pixel in img.pixels() {
        let (x, y) = (pixel.0, pixel.1);
        let _brightness = pixel.2.0[3];
        let (r, g, b): (f64, f64, f64) = (pixel.2.0[0].into(), pixel.2.0[1].into(), pixel.2.0[2].into());
        //let average = (r + g + b) / 3.0;

        // run pixel average through filter
        // only take x_modifier column and y_modifier row
        if y % y_modifier == 0 && x % x_modifier == 0 {
            let chara: char;
            match filter_input {
                "gray-basic" => chara = filters::grayscale_basic((r, g, b), reverse),
                "gray-detailed" => chara = filters::grayscale_detailed((r, g, b), reverse),
                "redgreen-basic" => chara = filters::redgreen_basic((r, g, b), reverse),
                _ => panic!("Filter {} does not exist", filter_input)
            };

            //println!("{}", filters::grayscale_detailed(average, reverse));

            result_before.push(chara);
        }

        // end each row with newline
        if x == img.dimensions().0 -1 {
            result_before.push_str("\n");
        }
    }

    // --- Re-Format result to not include blank line ---
    
    let mut result_final = String::from("");
    let mut last_char: char = ' ';

    // remove new line only lines
    for c in result_before.chars() {
        if !(c == last_char && c == '\n') {
            result_final.push(c);
        }
        last_char = c;
    }

    // --- Write final result into output.txt ---

    match file.write_all(result_final.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Wrote File"),
    }
}


*/