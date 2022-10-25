use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(module = "/src/runtime.js")]
extern "C" {
    pub type Application;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Application;

    #[wasm_bindgen(method, js_name = load)]
    pub fn load(this: &Application, url: String);
}
