extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen(start)]
fn main() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn secret(key: &str) -> String {
    if key != "this is secret!" {
        return String::from("wrong");
    }

    String::from("match")
}
