use compiler::{*, js::{JsTranspiler, JsTranspilerOptions}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let compiler = JsTranspiler::new(JsTranspilerOptions);

    match compiler.compile(input) {
        Ok(v) => v,
        Err(e) => format!("{:?}", e),
    }
}
