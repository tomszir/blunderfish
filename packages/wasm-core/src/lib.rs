#![allow(non_snake_case, non_upper_case_globals)]
use wasm_bindgen::prelude::wasm_bindgen;

pub mod engine;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
