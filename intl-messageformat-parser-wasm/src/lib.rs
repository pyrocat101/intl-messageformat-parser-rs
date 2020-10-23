mod utils;

use intl_messageformat_parser_rs::Parser;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, intl-messageformat-parser-wasm!");
// }

#[wasm_bindgen]
pub fn parse(message: String) -> Result<JsValue, JsValue> {
    let mut parser = Parser::new(message.as_str(), None);
    let parse_result = parser.parse().map_err(|_| JsValue::from_str("Invalid message"))?;
    Ok(JsValue::from_serde(&parse_result).unwrap())
}
