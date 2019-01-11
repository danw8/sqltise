#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate wasm_bindgen_test;

use wasm_bindgen::prelude::*;

pub mod generate;
mod model;
pub mod process;

use self::model::{ColumnHeader, CsvHeaders};
pub use self::process::process_file;

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
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

	for (index, column) in header_value.iter().enumerate() {
		let header = ColumnHeader {
			name: column.to_string(),
			index,
		};
		headers.push(header);
	}

	let headers = CsvHeaders { columns: headers };

	return JsValue::from_serde(&headers).unwrap();
}
