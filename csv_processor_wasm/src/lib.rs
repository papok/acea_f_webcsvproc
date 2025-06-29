use wasm_bindgen::prelude::*;
use acea_f_helpers::process_income_lines::main::process_csv_str;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}
#[wasm_bindgen]
pub fn process_csv(input: &str) -> Result<String, JsValue> {
    log("Processing CSV data...(2)");
    
    // Here you would implement your CSV processing logic.
    // For demonstration, we'll just return the input as output.
    
    // Ok(input.to_string())
    process_csv_str(input).map_err(|e| JsValue::from_str(&e.to_string()))
}   