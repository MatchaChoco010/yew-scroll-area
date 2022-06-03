use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/highlight.js")]
extern "C" {
    #[wasm_bindgen(js_name = highlight)]
    pub fn highlight();
}
