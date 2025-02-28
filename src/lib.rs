use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn calcloc(curloc: &str) -> String {
    format!("The current location is {}", curloc)
}

#[wasm_bindgen]
pub fn greet(txt: &str) {
    log(&format!("Hello! {}", txt));
}
