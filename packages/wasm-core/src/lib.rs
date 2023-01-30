#![allow(non_snake_case, non_upper_case_globals)]
use wasm_bindgen::prelude::wasm_bindgen;

mod utils;

#[wasm_bindgen]
pub fn init_debug() {
    utils::set_panic_hook();
}
