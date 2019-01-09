#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use std::io;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Jared, {}!", name));
}

#[wasm_bindgen]
pub fn get_columns(file_data: &str) -> JsValue {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_data.as_bytes());

    let mut headers = Vec::new();

    let header_value = match reader.headers() {
        Ok(h) => h,
        Err(_e) => {
            return JsValue::NULL;
        }
    };

    for column in header_value.iter() {
        headers.push(column.to_string());
    }

    let headers = CsvHeaders {
        columns: headers,
    };

    return JsValue::from_serde(&headers).unwrap();
}

#[derive(Serialize)]
struct CsvHeaders {
    columns: Vec<String>,
}