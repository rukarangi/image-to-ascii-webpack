pub mod filters;

pub enum Pixel_type {
    Gray,
    Rgb,
    GrayA,
    Rgba,
}

pub fn handle(bytes: Vec<u8>, pixel_type: Pixel_type, y_modifier: u32, x_modifier: u32, width: u32, height: u32) -> String {
    
    let mut result = String::new();

    match pixel_type {
        Pixel_type::Rgba => {
            result = handle_gray(bytes, y_modifier, x_modifier, width, height);
        },
        Pixel_type::Gray => {
            result = handle_gray(bytes, y_modifier, x_modifier, width, height);
        },
        _ => ()
    }
    
    
    return result;
}

pub fn handle_gray(bytes: Vec<u8>, y_modifier: u32, x_modifier: u32, width: u32, height: u32) -> String {
    let mut result = String::new();

    let mut row: u32 = 0;

    for (i, b) in bytes.iter().enumerate() {
        let col_mod = (i - (row * width) as usize) % x_modifier as usize;
        let row_mod = row % y_modifier;
        if i == 0 || *b == 0 as u8 || col_mod != 0  { // || row % y_modifier == 0
            // (i - (row * width) as usize) % x_modifier as usize == 0
            continue;
        }

        let mut next_str = String::from("");

        next_str.push(filters::grayscale_basic((*b as f64, *b as f64, *b as f64), true));

        if i % (width + 1) as usize == 0 {
            next_str.push_str(&format!("\n")[..]);
            //result.push_str(&format!("{:?}", row_mod)[..]);
            //result.push_str(&format!("{:?}", (i - (row * width) as usize))[..]);

            row += 1;
        }

        if row_mod != 0 {
            //next_str = String::from("");
            //continue;
        }

        //result.push(filters::grayscale_basic_test(*b as f64, true));


        if i % (width - 100) as usize == 0 {
            //result.push_str(&format!("{:?}", (i - (row * width) as usize & x_modifier as usize))[..]);
        }

        result.push_str(&next_str[..]);

    }

    // --- Re-Format result to not include blank line ---
    
    let mut result_final = String::from("");
    let mut last_char: char = ' ';

    // remove new line only lines
    for c in result.chars() {
        if !(c == last_char && c == '\n') {
            result_final.push(c);
        }
        last_char = c;
    }

    return result;

} 

pub fn handle_rgba(bytes: Vec<u8>) -> String {
    
    let mut result = String::new();
    let mut result_test = String::new();
    
    let mut pixels: Vec<(f64, f64, f64)>;

    let mut test: bool = true;

    let mut pos = 0;
    let mut next_tup: (f64, f64, f64);
    let mut temp: Vec<f64> = Vec::new();
    for (i, b) in bytes.iter().enumerate() {
        
        if i == 0 { //|| i % 4 == 0
            continue;
        }

        if pos == 3 {
            pos = 0;
            result.push(filters::grayscale_basic((temp[0], temp[1], temp[2]), true));
            
            result_test.push_str(&format!("{:?}", (temp[0], temp[1], temp[2]))[..]);
            temp = vec![];
        }

        if i % 1483 == 0 {
            result.push_str("\n");
        }

        // if test && i > 100  {
        //     test  = false;

        //     result.push_str(&format!("sample: {:?}", temp)[..]);

        // }

        temp.push(*b as f64);
        pos += 1;
    }
    
    return result_test;
} 

// log(&format!("{:X?}", self.data_decoded.len()));

        // let mut new_result = String::from("");

        // let mut last: u8 = 0;
        // let mut column: u32 = 0;
        // let width = as_u32_be(&self.png.ihdr.width[..]);
        // let mut row: u32 = 0;

        // for (idx, value) in self.data_decoded.iter().enumerate() {
        //     if last != *value {
        //         //log(&format!("{:?}", value));
        //     }
        //     last = *value;
        //     if column % x_modifier == 0 && row % y_modifier == 0 {
        //         let chara = filters::grayscale_basic_test(*value as f64, true);
        //         new_result.push(chara);
        //     }
        //     column += 1;
        //     if column >= width - 2 {
        //         new_result.push_str("\n");
        //         column = 0;
        //         row += 1;
        //         log("new line");
        //     }
        // }

        // let mut row = 1;
        // let mut column = 1;
        

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

        // new_result.push('a');

        // let mut result_final = String::from("");
        // let mut last_char: char = ' ';

        // // for c in new_result.chars() {
        // //     if !(c == last_char && c == '\n') {
        // //         result_final.push(c);
        // //     }
        // //     last_char = c;
        // // }

        // log("based result?:");
        // //log(&new_sresult[..]);
        // //self.result = new_result;
        // return new_result;