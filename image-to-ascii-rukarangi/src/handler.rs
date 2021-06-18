pub mod filters;
use std::convert::From;
use wasm_bindgen::prelude::*;
use image::Pixel;

#[wasm_bindgen]
extern {
    //fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn handle_new(bytes: Vec<u8>, y_modifier: u32, x_modifier: u32) -> String {
    let mut result = String::new();
    let img = image::load_from_memory(&bytes[..]); 
    // this is where I cheat

    match img {
        Ok(r) => {
            let i = r;
            let rgbs_ = i.to_rgb8();
            // Hopefully ill be able to write this step myself
            result = oldhandle(rgbs_, y_modifier, x_modifier);
        },
        Err(e) => log(&format!("{}", e)),
    }

    result
}

pub fn oldhandle(img: image::RgbImage, y_modifier: u32, x_modifier: u32) -> String {
    let mut result_before = String::from("");

    let mut row: u32 = 0;
    let mut column: u32;

    let mut _count = 0;
    for (i, pixel) in img.pixels().enumerate() {
        if i as u32 % img.dimensions().0 == 0 && i > 0 {
            row += 1;
        }
        column = i as u32 - (row * img.dimensions().0);
        let (x, y) = (column, row);

        if y % y_modifier == 0 && x % x_modifier == 0 {
            let tuplet = pixel.channels4();
            let rgb = (tuplet.0 as f64, tuplet.1 as f64, tuplet.2 as f64, tuplet.3 as f64);

            result_before.push(filters::grayscale_basic(rgb, true));
        }

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

    return result_final;
}