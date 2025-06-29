use wasm_bindgen::prelude::*;
use acea_f_helpers::process_income_lines::main::process_csv_str;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}
#[wasm_bindgen]
pub fn process_csv(input: &str) -> Result<JsValue, JsValue> {
    log("Processing CSV data...(2)");

    // Call your processing function and convert the result to JsValue (e.g., as a JS array)
    match process_csv_str(input) {
        Ok(arr) => Ok(serde_wasm_bindgen::to_value(&arr).unwrap_or(JsValue::UNDEFINED)),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}