use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    pub type Spline;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Spline;

    #[wasm_bindgen(method)]
    pub fn run(this: &Spline, url: String);
}