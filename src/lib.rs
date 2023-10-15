use compiler::*;
use volt::tree::SyntaxDisplay;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let compiler = JsTranspiler::new(());

    match compiler.parse(input) {
        Ok(tree) => tree.fmt(0).iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n"),
        Err(e) => format!("{:?}", e),
    }
}
