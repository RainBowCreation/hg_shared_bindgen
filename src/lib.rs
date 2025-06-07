use wasm_bindgen::prelude::*;
use hg_shared_core::{ hash_sha256, add_numbers };

#[wasm_bindgen]
pub fn hash_sha256_wasm(input: &str) -> String {
    hash_sha256(input)
}

#[wasm_bindgen]
pub fn add_numbers_wasm(a: i32, b: i32) -> i32 {
    add_numbers(a, b)
}