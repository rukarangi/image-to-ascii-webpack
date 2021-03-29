pub mod filters;
use std::fmt;
use std::convert::From;
use wasm_bindgen::prelude::*;
use image::Pixel;

#[wasm_bindgen]
extern {
    //fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn handle_new(bytes: &Vec<u8>, y_modifier: u32, x_modifier: u32) -> String {
    let mut result = String::new();

    //let img = image::load_from_memory(&bytes[..])?;
    let img = image::load_from_memory(&bytes[..]);
    //let rgb = img.to_rgb8();
    let rgb: u8;

    match img {
        Ok(r) => {
            let i = r;
            let rgbs_ = i.to_rgb8();
            result = oldhandle(rgbs_, y_modifier, x_modifier);
        },
        Err(e) => log(&format!("{}", e)),
    }

    result
}

pub fn oldhandle(img: image::RgbImage, y_modifier: u32, x_modifier: u32) -> String {
    let mut result_before = String::from("");

    let mut row: u32 = 0;
    let mut column: u32 = 0;

    let mut _count = 0;
    for (i, pixel) in img.pixels().enumerate() {
        if i as u32 % img.dimensions().0 == 0 && i > 0 {
            row += 1;
        }
        column = i as u32 - (row * img.dimensions().0);
        let (x, y) = (column , row);
        //let average = (r + g + b) / 3.0;

        // run pixel average through filter
        // only take x_modifier column and y_modifier row
        if y % y_modifier == 0 && x % x_modifier == 0 {
            let chara: char;
            

            let tuplet = pixel.channels4();
            let rgb = (tuplet.0 as f64, tuplet.1 as f64, tuplet.2 as f64, tuplet.3 as f64);

            chara = filters::grayscale_basic(rgb, true);
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

    return result_final;
}

pub enum Pixel_type {
    Gray,
    Rgb,
    GrayA,
    Rgba,
}

pub struct Pixel_ {
    r: u8,
    g: u8,
    b: u8
}

#[derive(Copy, Clone)]
pub struct PixelA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl From<Vec<u8>> for PixelA {
    fn from(v: Vec<u8>) -> PixelA {
        PixelA { 
            r: v[0],
            g: v[1],
            b: v[2],
            a: v[3]
        }
    }
}

impl From<PixelA> for Vec<u8> {
    fn from(p: PixelA) -> Vec<u8> {
        vec![p.r, p.g, p.b, p.a]
    }
}

impl fmt::Display for Pixel_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Display for PixelA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

pub fn handle(bytes: Vec<u8>, pixel_type: Pixel_type, y_modifier: u32, x_modifier: u32, width: u32, height: u32) -> String {
    
    let mut result = String::new();

    match pixel_type {
        Pixel_type::Rgba => {
            result = handle_rgb(bytes, y_modifier, x_modifier, width, height);
        },
        Pixel_type::Gray => {
            result = handle_gray(bytes, y_modifier, x_modifier, width, height);
        },
        _ => ()
    }
    
    
    return result;
}

pub fn rgb_maker(bytes: Vec<u8>) -> Vec<Pixel_> {
    let mut pixels: Vec<Pixel_> = Vec::new();
    
    for (i, b) in bytes.iter().enumerate() {
        if i % 4 as usize != 0 || i == 0 {
            continue;
        }

        let new_rgb: Pixel_ = Pixel_ {
            r: bytes[i - 2], 
            g: bytes[i - 1], 
            b: bytes[i]
        };

        pixels.push(new_rgb);

    }

    return pixels;
}

pub fn remove_blank(input: String) -> String {
    let mut result = String::from("");
    let mut last_char: char = ' ';

    // remove new line only lines
    for c in input.chars() {
        if !(c == last_char && c == '\n') {
            result.push(c);
        }
        last_char = c;
    }

    return result;
}

pub fn rgba_maker(bytes: Vec<u8>, width: u32) -> Vec<PixelA> {
    let mut pixels: Vec<PixelA> = Vec::new();
    let mut offset = 0;

    let mut i_: usize = 0;

    let mut new_bytes: Vec<u8> = vec![];

    let mut filter = bytes[0];
    let white = vec![
        (0xFF), 
        (0xFF), 
        (0xFF),
        (0xFF)
    ];
    let white_pixel: PixelA = PixelA::from(white);

    // pixels.push(white_pixel);
    // pixels.push(white_pixel);
    // pixels.push(white_pixel);
    // pixels.push(white_pixel);


    for (i,b) in bytes.iter().enumerate() {
        if i % (width * 4) as usize != 0 {
            new_bytes.push(*b);
        }
    }

    for (i, b) in new_bytes.iter().enumerate() {
        if i % 4 == 0 && i != 0 {
            let new_rgb: Vec<u8> = vec![
                (new_bytes[i - 4]), 
                (new_bytes[i - 3]), 
                (new_bytes[i - 2]),
                (new_bytes[i - 1])
            ];

            let mut last_rgb: Vec<u8> = Vec::new();

            if (i % width as usize) > 9 {
                last_rgb = Vec::<u8>::from(pixels[i / new_bytes.len()]);

                // Trying all around to tget sub filter to work!!! ARGH
            } else {
                last_rgb = vec![
                    (0x0), 
                    (0x0), 
                    (0x0),
                    (0x0)
                ];
            }

            // if filter == 0 {
            //     pixels.push(PixelA::from(new_rgb));
            // } else {
            pixels.push(PixelA::from(sub_filter(4, last_rgb, new_rgb)));
            // }
        }
    }

    return pixels;
}

pub fn sub_filter(pixel_len: u32, last_pixel: Vec<u8>, current_pixel: Vec<u8>) -> Vec<u8> {
    let mut new_pixel: Vec<u8> = Vec::<u8>::new();

    for i in 0..pixel_len {
        new_pixel.push(current_pixel[i as usize].wrapping_add(last_pixel[i as usize]));
    }

    return new_pixel;
}

pub fn handle_gray(bytes: Vec<u8>, y_modifier: u32, x_modifier: u32, width_1: u32, height: u32) -> String {
    let mut result = String::new();

    let width = width_1 + 1;

    let mut row: u32 = 1;

    let mut rows: Vec<Vec<u8>>;
    let mut row_: Vec<u8> = vec![0;width as usize];

    for (i, b) in bytes.iter().enumerate() {
        let col_mod = (i - (row * width) as usize) % x_modifier as usize;
        let row_mod = row % y_modifier;
        if col_mod != 0 { //  i == 0 || || row % y_modifier == 0 || *b == 0 as u8 
            // (i - (row * width) as usize) % x_modifier as usize == 0
            continue;
        }

        let mut next_str = String::from("");

        //next_str.push(filters::grayscale_basic((*b as f64, *b as f64, *b as f64), true));
        //next_str.push_str(&format!("\n")[..]);

        if i % (width) as usize == 0 {
            next_str.push_str(&format!("\n")[..]);
            //result.push_str(&format!("{:?}", row_mod)[..]);
            //result.push_str(&format!("{:?}", (i - (row * width) as usize))[..]);

            row += 1;
        }

        if row_mod != 0 && row > 0 {
            //next_str = String::from("");
            result.push_str(&next_str[..]);
            continue;
        }

        //next_str.push_str(&format!("{:?} ", (i % (width as usize)))[..]);

        //result.push_str(&format!(" ({:X?}, {:?}, {:?}, ", *b, i, row)[..]);
        result.push(filters::grayscale_basic_test(*b as f64, true));
        //result.push(')');



        if i % (width - 100) as usize == 0 {
            //result.push_str(&format!("{:?}", (i - (row * width) as usize & x_modifier as usize))[..]);
        }

        result.push_str(&next_str[..]);

    }
    
    let mut result_final = remove_blank(result);

    return result_final;

}

pub fn handle_rgb(bytes: Vec<u8>, y_modifier: u32, x_modifier: u32, width_1: u32, height: u32) -> String {
    let pixels = rgba_maker(bytes, width_1);
    let mut result_1 = String::new();
    let mut result = String::new();

    let width = width_1 ;// + 1) * 1;

    let mut row: u32 = 1;

    for (i, p) in pixels.iter().enumerate() {
        result_1.push_str(&format!(" {:}", p)[..]);
        if i % width as usize == 0 {
            result_1.push_str(&format!("\n"));
        }
    }

    for (i, p) in pixels.iter().enumerate() {
        let col_mod = (i - (row * width) as usize) % x_modifier as usize;
        let row_mod = row % y_modifier;

        if col_mod !=0 {
            continue;
        }

        let mut next_str = String::from("");

        if i % (width) as usize == 0 {
            next_str.push_str(&format!("\n")[..]);
            row += 1;
        }
        
        if row_mod != 0 && row > 0 {
            //next_str = String::from("");
            result.push_str(&next_str[..]);
            continue;
        }

        let data = (p.r as f64, p.g as f64, p.b as f64, p.a as f64);
        //result.push_str(&format!(" ({:?}, {:?}, ", i, row)[..]);
        //result.push(filters::grayscale_basic(data, false));
        //result.push(')');
        result.push_str(&format!(" ({:X?}, {:X?}, {:X?})", p.r, p.g, p.b)[..]);
    }

    let mut result_final = remove_blank(result);

    return result_final;

} 