extern crate console_error_panic_hook;

mod utils;
mod filters;
mod parser;

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
}

#[wasm_bindgen]
impl Converter {
    pub fn filter(&mut self, y_modifier: u32, x_modifier: u32) -> String {

        log(&format!("{:X?}", self.data_decoded.len()));

        let mut new_result = String::from("");

        let mut last: u8 = 0;
        let mut column: u32 = 0;
        let mut row: u32 = 0;

        for (idx, value) in self.data_decoded.iter().enumerate() {
            if last != *value {
                //log(&format!("{:?}", value));
            }
            last = *value;
            if column % x_modifier == 0 && row % y_modifier == 0 {
                let chara = filters::grayscale_basic_test(*value as f64, true);
                new_result.push(chara);
            }
            column += 1;
            if *value == 0 {
                new_result.push_str("\n");
                column = 0;
                row += 1;
            }
        }

        // let mut row = 1;
        // let mut column = 1;
        // let width = as_u32_be(&self.png.ihdr.width[..]);

        // for (idx, value_1) in self.data_decoded.iter().enumerate() {
        //     let value: f64 = *value_1 as f64;
            
            
        //     row = idx as u32 - ((column as f64 / width as f64).floor() as u32 * width);
        //     log(&format!("{:?}", [width, column, row]));
        //     log(&format!("{:?}", [*value_1 as u32, (self.data_decoded[idx] as u32), (self.data_decoded.len() as u32)]));


        //     //if row % y_modifier == 0 && column % x_modifier == 0 {
        //     let chara: char;
        //     chara = filters::grayscale_basic_test(*value_1 as f64, true);
        //     new_result.push(chara);
        //     //}

        //     if value == 0.0 {
        //         return new_result.push_str("\n");
        //     }
        //     column += 1;
        // }

        new_result.push('a');

        let mut result_final = String::from("");
        let mut last_char: char = ' ';

        // for c in new_result.chars() {
        //     if !(c == last_char && c == '\n') {
        //         result_final.push(c);
        //     }
        //     last_char = c;
        // }

        log("based result?:");
        log(&new_result[..]);
        //self.result = new_result;
        return new_result;
    }

    pub fn new(data_raw: Vec<u8>) -> Converter {
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
        };
    }

    pub fn populate_ihdr(&mut self) {
        let index = self.find_pattern(parser::IHDR.to_vec());
        let mut ihdr_bytes: [u8; 13] = [0; 13];
        for i in 0..13 {
            ihdr_bytes[i] = self.data_raw[i+index];
        }
        log(&format!("{:X?}", ihdr_bytes)[..]);
        let ihdr = parser::IhdrChunk::build(ihdr_bytes);
        self.png.ihdr = ihdr;
    }

    pub fn populate_idat(&mut self) {
        let index = self.find_pattern(parser::IDAT.to_vec());
        log(&format!("found idat index: {:X?}", index)[..]);

        let slice = &self.data_raw[(index-8)..(index-4)];
        log(&format!("length bytes: {:X?} Idat: {:X}", slice, self.data_raw[index]));

        let mut idat_bytes: Vec<u8> = Vec::<u8>::new();
        let length_1: u32 = as_u32_be(slice);
        let length: u32 = if length_1 < self.data_raw.len() as u32 {length_1} else {10};
        log(&format!("found idat length: {}", length)[..]);

        let mut count = 0;
        loop {
            count += 1;
            idat_bytes.push(self.data_raw[count+index]);
            if count == length as usize {
                log("pushed to bytes");
                break;
            }
        }
        log(&format!("bytes {:X?}", idat_bytes)[..]);
        self.data_raw = idat_bytes;
    }

    pub fn decode_idat(&mut self) {
        //let decoded: Vec<u8> = inflate_bytes_zlib(&self.data_raw).unwrap();
        let data_1 = self.data_raw.clone();
        let data = &data_1[1..];
        let mut decoded = Vec::<u8>::new();

        let mut decoder = inflate::InflateWriter::new(Vec::new());
        match decoder.write(&data) {
            Ok(x) => log("decoder decoded"),
            Err(e) => log(&format!("decoder failed {}", e)[..])
        }
        match decoder.finish() {
            Ok(x) => decoded = x,
            Err(_) => decoded = vec![0x49]
        }

        log("Decoded Idat");
        log(&format!("bytes: {:X?}", &decoded)[..]);
        

        self.data_decoded = decoded.clone();

        log(&format!("bytes: {:X?}", &self.data_decoded)[..]);
    }

    pub fn display_head(&self) {
        log(&format!("Head Information:", )[..]);
        log(&format!("width: {:X?}", self.png.ihdr.width)[..]);
        log(&format!("height: {:X?}", self.png.ihdr.height)[..]);
        log(&format!("depth: {:X?}", self.png.ihdr.depth)[..]);
        log(&format!("color_type: {:X?}", self.png.ihdr.color_type)[..]);
        log(&format!("compression: {:X?}", self.png.ihdr.compression)[..]);
        log(&format!("filter: {:X?}", self.png.ihdr.filter)[..]);
        log(&format!("interlaced: {:X?}", self.png.ihdr.interlaced)[..]);
        log(&format!("total byte length: {:X?}", self.data_raw.len()));
    }

    pub fn test_pattern(&self) {
        self.find_pattern(parser::IHDR.to_vec());
    }

    pub fn find_pattern(&self, pattern: Vec<u8>) -> usize {
        let mut index: usize = 0;

        for (idx, byte) in self.data_raw.iter().enumerate() {
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
        return index;
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