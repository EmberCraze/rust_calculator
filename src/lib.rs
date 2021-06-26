mod utils;
mod parsing;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert("Hello, calculator-wasm!");
    alert(name);
}

#[wasm_bindgen]
pub fn parse_input(data: &str) -> String{
    //alert("Hello, calculator-wasm!");
    //alert(data);
    let postfix: Vec<String> = parsing::to_postfix(&data.to_string());
    return parsing::execute_postfix(postfix).unwrap();
}
