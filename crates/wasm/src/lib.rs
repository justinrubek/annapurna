use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn init_wasm() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn check_recipes() -> Result<String, JsValue> {
    /*
    let parser = ProgramParser::new();
    let parsed = parser
        .parse(&code)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let compiled = generate_program(parsed).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(compiled)
    */

    Ok("Hello, World!".to_string())
}
