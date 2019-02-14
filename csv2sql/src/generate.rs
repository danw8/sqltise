use super::log;
use super::model::{
	ColumnHeader, ColumnType, CsvErrors, CsvError, StatementSelections, StatementSelection, StatementType, ColumnSelection, ColumnSource,
};

use csv::StringRecord;
use super::{DATETIME_FORMATS, DATE_FORMATS};
use chrono::{NaiveDate, NaiveDateTime};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_file(data: &str, statements: JsValue, corrections: JsValue, combine: bool) -> JsValue {
	
	let combine_text = format!("combine: {}", combine);
	log(&combine_text);

	let statements: StatementSelections = match statements.into_serde() {
		Ok(s) => s,
		Err(_e) => {
			return JsValue::from_str("Couldn't deserialize the statements, did the model change?");
		}
	};

	let corrections: CsvErrors = match corrections.into_serde() {
		Ok(c) => c,
		Err(_) => {
			return JsValue::from_str("Couldn't deserialize the Errors, did the model change?");
		}
	};

	let results = match generate_statements(data, statements.value, corrections.value, combine) {
		Ok(sql) => sql,
		Err(e) => return JsValue::from_str(&e),
	};

	return JsValue::from_serde(&results).unwrap();
}

fn get_headers(reader: &mut csv::Reader<&[u8]>) -> Result<Vec<ColumnHeader>,String>  {
	let mut headers = Vec::new();

	let header_value = match reader.headers() {
		Ok(h) => h,
		Err(_e) => {
			return Err("Couldn't read the headers".to_string());
		}
	};

	for (index, column) in header_value.iter().enumerate() {
		let header = ColumnHeader {
			name: column.to_string(),
			index,
		};
		headers.push(header);
	}

	Ok(headers)
}

fn generate_statements(data: &str, statements: Vec<StatementSelection>, corrections: Vec<CsvError>, combine: bool) -> Result<Vec<String>, String> {
	let mut reader = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_reader(data.as_bytes());

	let headers = get_headers(&mut reader)?;

	let mut sql_by_rows = Vec::<(usize, String)>::new();

	for(index, row) in reader.records().enumerate() {

		let record = row
			.map_err(|e| 
			format!("The .csv file could not be parsed. Internal Error: {}", e ))?;

		if record.iter().all(|r| r.trim().is_empty()) {
			continue;
		}

		for statement in &statements {
			let sql = get_row_sql(statement, &record, index,  &corrections, &headers);
			sql_by_rows.push((statement.id, sql));
		}
	}

	let mut files = Vec::new();

	if combine {
		let num_statements = statements.len();
		let rows = sql_by_rows.chunks(num_statements);
		let output = rows
			.filter(|a| a.len() != 0)
			.map(|row| row.iter().map(|(_, v)| v.to_string()).collect::<Vec<String>>().join("\n"))
			.collect::<Vec<String>>()
			.join("\n GO \n");
		files.push(output);
	} else {
		for statement in &statements {
			let output = sql_by_rows.iter()
				.filter(|(k, _)| k == &statement.id)
				.map(|(_, v)| v.to_string())
				.collect::<Vec<String>>()
				.join("\n");
			files.push(output);
		}
	};

	Ok(files)
}

fn get_row_sql(statement: &StatementSelection, record: &StringRecord, index: usize, corrections: &Vec<CsvError>, headers: &Vec<ColumnHeader>) -> String {
	let mut outputs = Vec::new();
	for column in &statement.column_selections.value {
		match column.source {
			ColumnSource::FreeText => {
				let name: String = column.name.clone().unwrap();
				let mut value: String = column.data.clone().unwrap();
				value = format_value(column.r#type.clone(), &value);
				outputs.push((name, value));
			},
			ColumnSource::CSV => {
				outputs.push(get_column_value(statement.id, index, column, record, corrections, headers));
			}
		}

	}

	match statement.r#type {
		StatementType::Insert => {
			get_insert_statement(&outputs, statement)
		},
		StatementType::Update => {
			get_update_statement(outputs, statement, record, index, corrections)
		}
	}
}

fn get_update_statement(values: Vec<(String, String)>, statement: &StatementSelection, record: &StringRecord, index: usize, corrections: &Vec<CsvError>) -> String{
	let sets = values
		.iter()
		.map(|(c, v)| format!("{} = {}", c, v))
		.collect::<Vec<String>>()
		.join(", ");

	let mut where_clause = String::new();

	for (pos, condition) in statement.where_selections.iter().enumerate() {
		let where_column_id = condition.value.unwrap();
		let mut column_value = record[where_column_id].to_string();

		if let Some(correction) = corrections.iter().find(|c| {
			c.column_id == where_column_id
				&& c.statement_id == statement.id
				&& c.rows.iter().any(|&r| r == index)
		}) {
			column_value = correction.error_text.clone();
		}
		column_value = format_value(condition.r#type.clone().unwrap(), &mut column_value);

		let condition_text = if pos == 0 { "" } else { "AND "};
		let compare_text = if column_value == "NULL" { "IS" } else { "="};

		where_clause = format!("{} {} {} {} {}", where_clause, condition_text, condition.key, compare_text, column_value);
	}

	format!(
		"UPDATE {} SET {} WHERE {};",
		statement.table, sets, where_clause
	)
}

fn get_insert_statement(values: &Vec<(String, String)>, statement: &StatementSelection) -> String{
	let column_names = values
		.iter()
		.map(|(c, _)| c.clone())
		.collect::<Vec<String>>()
		.join(", ");
	let values = values
		.iter()
		.map(|(_, v)| v.clone())
		.collect::<Vec<String>>()
		.join(", ");
	format!(
		"INSERT INTO {} ({}) VALUES ({});",
		statement.table, column_names, values
	)
}


fn get_column_value(statement_id: usize, index: usize, column: &ColumnSelection, record: &StringRecord, corrections: &Vec<CsvError>, headers: &Vec<ColumnHeader> ) -> (String, String) {
		let id = column.column;
		let mut value: String = record[id].to_string();

		if let Some(correction) = corrections.iter().find(|c| {
			c.column_id == id
				&& c.statement_id == statement_id
				&& c.rows.iter().any(|&r| r == index)
		}) {
			value = correction.error_text.clone();
		}

		let name = match column.use_source {
			true => headers.iter().find(|h| h.index == id).unwrap().name.clone(),
			false => column.name.clone().unwrap(),
		};

		value = format_value(column.r#type.clone(), &value);
		(name, value.to_string())

}

fn parse_datetime(value: &str) -> Result<NaiveDateTime, String> {
	let value = value.trim();
	for format in &DATETIME_FORMATS {
		let parsed = NaiveDateTime::parse_from_str(value, format);
		if parsed.is_ok() {
			return Ok(parsed.unwrap());
		}
	}

	for format in &DATE_FORMATS {
		let parsed = NaiveDate::parse_from_str(value, format);
		if parsed.is_ok() {
			return Ok(parsed.unwrap().and_hms(0, 0, 0));
		}
	}

	log("No Formats Match");
	Err("No Formats Match".to_string())
}

fn is_null(value: &str) -> bool {
	let value = value.trim();
	value.to_lowercase() == "null"
}


fn format_value(column_type: ColumnType, value: &str) -> String{
	let mut new_value = value.to_string(); 
	match column_type {
		ColumnType::VarChar => {
			if !is_null(&value) {
				new_value = str::replace(&value, "'", "''");
				new_value = format!("'{}'", new_value);
			} else {
				new_value = "NULL".to_string();
			}
		}
		ColumnType::Date => {
			if !is_null(&value) {
				let date = parse_datetime(&value);
				if date.is_ok() {
					new_value = date.unwrap().format("%Y-%m-%d").to_string();
				}
				new_value = format!("'{}'", new_value);
			} else {
				new_value = "NULL".to_string();
			}
		}
		ColumnType::DateTime => {
			if !is_null(&value) {
				let date = parse_datetime(&value);
				if date.is_ok() {
					new_value = date.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
				}
				new_value = format!("'{}'", new_value);
			} else {
				new_value = "NULL".to_string();
			}
		}
		_ => {}
	}

	new_value
}