#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

mod model;

use self::model::{CsvHeaders, StatementSelections, ColumnSelections, ColumnHeader};

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

	for (index, column) in header_value.iter().enumerate() {
		let header = ColumnHeader{
			name: column.to_string(),
			index
		};
		headers.push(header);
	}

	let headers = CsvHeaders {
		columns: headers,
	};

	return JsValue::from_serde(&headers).unwrap();
}

#[wasm_bindgen]
pub fn process_file(_data: &str, statements: JsValue, columns: JsValue) -> JsValue {
	let _statements: StatementSelections = statements.into_serde().unwrap();
	let _columns: ColumnSelections = columns.into_serde().unwrap();

	JsValue::NULL
}
