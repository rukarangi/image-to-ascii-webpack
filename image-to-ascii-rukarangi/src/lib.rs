extern crate console_error_panic_hook;

mod utils;
mod handler;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use wasm_bindgen::prelude::*;
use std::panic;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
pub fn different_methods(data_raw: Vec<u8>, y_modifier: u32, x_modifier: u32) -> String {
    let result = handler::handle_new(data_raw, y_modifier, x_modifier);

    return result;
}
