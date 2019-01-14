use super::log;
use super::model::{
	ColumnType, CsvError, CsvErrors, ParseError, StatementSelections, StatementType,
};
use super::{DATETIME_FORMATS, DATE_FORMATS};
use chrono::{NaiveDate, NaiveDateTime};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check_correction(value: &str, column_type: &str) -> JsValue {
	let error = match column_type {
		"Int" => check_int_errors(value),
		"Float" => check_float_errors(value),
		"Date" => check_date_errors(value),
		"DateTime" => check_date_errors(value),
		"VarChar" => check_varchar_errors(value),
		_ => false,
	};
	return JsValue::from_bool(!error);
}

#[wasm_bindgen]
pub fn process_file(data: &str, statements: JsValue) -> JsValue {
	let statements: StatementSelections = statements.into_serde().unwrap();
	//let columns: ColumnSelections = columns.into_serde().unwrap();

	let mut errors: Vec<CsvError> = Vec::new();

	for statement in statements.value {
		let statement_id = statement.id;

		let mut reader = csv::ReaderBuilder::new()
			.has_headers(true)
			.from_reader(data.as_bytes());

		for (index, row) in reader.records().enumerate() {
			let mut column_errors: Vec<CsvError> = Vec::new();

			let record = match row {
				Ok(r) => r,
				Err(e) => {
					let error = &format!("{{ error: 'Not a proper csv file', kind: {} }}", e);
					let json: ParseError = serde_json::from_str(error).unwrap();
					return JsValue::from_serde(&json).unwrap();
				}
			};

			if record.iter().all(|r| r.trim().is_empty()) {
				continue;
			}

			for column in &statement.column_selections.value {
				let id = column.column;

				let value = &record[id];

				let error: bool = match &column.r#type {
					ColumnType::Int => check_int_errors(value.trim()),
					ColumnType::Float => check_float_errors(value.trim()),
					ColumnType::Date => check_date_errors(value.trim()),
					ColumnType::DateTime => check_date_errors(value.trim()),
					ColumnType::VarChar => check_varchar_errors(value.trim()),
				};

				if error {
					if let Some(e) = column_errors
						.iter_mut()
						.find(|i| &(**i).error_text == value)
					{
						e.rows.push(index);
					} else {
						column_errors.push(CsvError {
							statement_id,
							column_id: id,
							r#type: column.r#type.clone(),
							error_text: value.to_string(),
							rows: vec![index],
						});
					}
				}
			}
			errors.append(&mut column_errors);

			if statement.r#type == StatementType::Update {
				let mut where_errors: Vec<CsvError> = Vec::new();
				let where_column_id = statement.r#where.value.unwrap();
				let where_column_type = statement.r#where.r#type.clone().unwrap();
				let value = &record[where_column_id];

				let error: bool = match where_column_type {
					ColumnType::Int => check_int_errors(value.trim()),
					ColumnType::Float => check_float_errors(value.trim()),
					ColumnType::Date => check_date_errors(value.trim()),
					ColumnType::DateTime => check_date_errors(value.trim()),
					ColumnType::VarChar => check_varchar_errors(value.trim()),
				};

				if error {
					if let Some(e) = where_errors.iter_mut().find(|i| &(**i).error_text == value) {
						e.rows.push(index);
					} else {
						where_errors.push(CsvError {
							statement_id,
							column_id: where_column_id,
							r#type: where_column_type.clone(),
							error_text: value.to_string(),
							rows: vec![index],
						});
					}
				}

				errors.append(&mut where_errors);
			}
		}
	}

	let csv_error = CsvErrors { value: errors };
	return JsValue::from_serde(&csv_error).unwrap();
}

fn check_int_errors(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return false;
	}
	!value.parse::<i32>().is_ok()
}

fn check_float_errors(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return false;
	}
	!value.parse::<f32>().is_ok()
}

fn check_date_errors(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return false;
	}

	for format in &DATETIME_FORMATS {
		let parsed = NaiveDateTime::parse_from_str(value.trim(), format);
		if parsed.is_ok() {
			return false;
		}
	}

	for format in &DATE_FORMATS {
		let parsed = NaiveDate::parse_from_str(value.trim(), format);
		if parsed.is_ok() {
			return false;
		}
	}

	return true;
}

fn check_varchar_errors(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return false;
	}
	false
}

fn is_null(value: &str) -> bool {
	let value = value.trim();
	value.to_lowercase() == "null"
}

#[cfg(test)]
pub mod tests {

	use super::super::model::{ColumnSelections, CsvError, CsvErrors};
	use super::process_file;
	use wasm_bindgen::prelude::*;

	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn process_file_finds_errors() {
		let (csv_data, column_selections) = setup();
		let value = process_file(&csv_data, column_selections);
		let errors: CsvErrors = value.into_serde().unwrap();
		assert_eq!(2, errors.value.len());
	}

	fn setup() -> (String, JsValue) {
		let csv_data = r#"FirstName,Enabled,StartDate
Dan,1,01/01/2017
Carlos,0,2017-01-01
Gerald,h,Not a date
Jordan,2,01/02/2016
"#;

		let column_selections = r#"
{
 "value": [
    {
      "column": 0,
      "statement_id": 0,
      "name": "FirstName",
      "type": "VarChar",
      "use_source": true
    },
    {
      "column": 1,
      "statement_id": 0,
      "name": "Enabled",
      "type": "Int",
      "use_source": true
    },
    {
      "column": 2,
      "statement_id": 0,
      "name": "StartDate",
      "type": "Date",
      "use_source": true
    }
  ],
  "done": false
}
"#;
		let column_selections: ColumnSelections = serde_json::from_str(column_selections).unwrap();
		let column_selections: JsValue = JsValue::from_serde(&column_selections).unwrap();

		(csv_data.to_string(), column_selections)
	}
}
