mod utils;

use diff::get_owned_diff;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn diff_text(a: &str, b: &str) -> JsValue {
    let res = get_owned_diff(a, b);
    JsValue::from_serde(&res).unwrap()
}
