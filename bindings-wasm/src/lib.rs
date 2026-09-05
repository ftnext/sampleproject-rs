use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(number: i32) -> i32 {
    sample_core::add_one(number)
}
