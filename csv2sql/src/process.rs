//super::log;
use super::model::{
	ColumnType, CsvError, CsvErrors, StatementSelections, StatementType, StatementSelection, ColumnSource
};
use super::{DATETIME_FORMATS, DATE_FORMATS};
use chrono::{NaiveDate, NaiveDateTime};
use wasm_bindgen::prelude::*;
use csv::StringRecord;

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
	let statements: StatementSelections = match statements.into_serde() {
		Ok(s) => s,
		Err(_e) => {
			return JsValue::from_str("Couldn't deserialize the statements, did the model change?");
		}
	};

	let errors = match process_file_impl(data, statements) {
		Ok(e) => e,
		Err(e) => return JsValue::from_str(&e),
	};

	let csv_error = CsvErrors { value: errors };
	return JsValue::from_serde(&csv_error).unwrap();
}


fn process_file_impl(data: &str, statements: StatementSelections) -> Result<Vec<CsvError>, String> {
	let mut errors: Vec<CsvError> = Vec::new();

	let mut reader = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_reader(data.as_bytes());

	for (index, row) in reader.records().enumerate() {

		let record = row
			.map_err(|e| 
			format!("The .csv file could not be parsed. Internal Error: {}", e ))?;

		if record.iter().all(|r| r.trim().is_empty()) {
			continue;
		}

		for statement in &statements.value {
			process_record_for_statment(&record, index, &statement, &mut errors);

			if statement.r#type == StatementType::Update {
				for condition in &statement.where_selections {
					let column_id = condition.value.unwrap();
					let column_type = condition.r#type.clone().unwrap();
					let value = &record[column_id];

					if check_for_error(&column_type, value) {
						add_error(&mut errors, value, statement.id, column_id, index, &column_type);
					}
				}
			}
		}
	}

	Ok(errors)
}


fn process_record_for_statment(record: &StringRecord, index: usize,  statement: &StatementSelection, mut errors: &mut Vec<CsvError>){
	for column in &statement.column_selections.value {
		match column.source {
			ColumnSource::FreeText => {
				// Don't add errors for freetext it always is what the user typed in.
			},
			ColumnSource::CSV => {
				let id = column.column;
				let value = &record[id];

				if check_for_error(&column.r#type, value) {
					add_error(&mut errors, value, statement.id, id, index, &column.r#type);
				}
			}
		}
	}
}

fn add_error(errors: &mut Vec<CsvError>,
	value: &str,
	statement_id: usize,
	column_id: usize,
	index: usize,
	column_type: &ColumnType)
{
	if let Some(e) = errors
		.iter_mut()
		.find(|i| &(**i).error_text == value && &(**i).column_id == &column_id) {
			e.rows.push(index);
		}
	else {
		let error =	CsvError {
			statement_id,
			column_id,
			r#type: column_type.clone(),
			error_text: value.to_string(),
			rows: vec![index],
		};
		errors.push(error);
	}
}

fn check_for_error(column_type: &ColumnType, value: &str) -> bool {
	match column_type {
		ColumnType::Int => check_int_errors(value.trim()),
		ColumnType::Float => check_float_errors(value.trim()),
		ColumnType::Date => check_date_errors(value.trim()),
		ColumnType::DateTime => check_date_errors(value.trim()),
		ColumnType::VarChar => check_varchar_errors(value.trim()),
		ColumnType::PerFormatted => true,
	}
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
