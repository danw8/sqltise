pub use crate::error::*;
pub use crate::statement::*;
use csv::StringRecord;

pub struct ValidationError {
	pub column: usize,
	pub rows: Vec<usize>,
	pub value: String,
}

pub fn validate(
	data: &str,
	statements: Vec<Statement>,
) -> Result<Vec<ValidationError>, SqltiseError> {
	let mut errors = vec![];

	let mut reader = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_reader(data.as_bytes());

	for (index, row) in reader.records().enumerate() {
		let record = row.map_err(|_| SqltiseError::CsvParseError)?;
		if record.iter().all(|r| r.trim().is_empty()) {
			continue;
		}
		for statement in &statements {
			check_row_for_errors(statement, index, &record, &mut errors)?;
		}
	}

	Ok(errors)
}

fn check_row_for_errors(
	statement: &Statement,
	index: usize,
	row: &StringRecord,
	errors: &mut Vec<ValidationError>,
) -> Result<(), SqltiseError> {
	match &statement.info {
		StatementInfo::Insert { columns, .. } => {
			check_column_errors(index, row, columns, errors)?;
		}
		StatementInfo::Update {
			columns,
			conditions,
			..
		} => {
			check_column_errors(index, row, columns, errors)?;
			check_condition_errors(index, row, conditions, errors)?;
		}
		StatementInfo::Custom { .. } => {}
	}
	Ok(())
}

fn check_condition_errors(
	index: usize,
	row: &StringRecord,
	conditions: &Vec<Condition>,
	errors: &mut Vec<ValidationError>,
) -> Result<(), SqltiseError> {
	for condition in conditions {
		let value = match row.get(condition.source) {
			Some(v) => v,
			None => return Err(SqltiseError::ColumnOutOfBounds),
		};

		if !validate_data(condition.sql_type, value) {
			add_validation_error(errors, value, condition.source, index);
		}
	}
	Ok(())
}

fn check_column_errors(
	index: usize,
	row: &StringRecord,
	columns: &Vec<Column>,
	errors: &mut Vec<ValidationError>,
) -> Result<(), SqltiseError> {
	for column in columns {
		match column {
			Column::Csv {
				source, sql_type, ..
			} => {
				let value = match row.get(*source) {
					Some(v) => v,
					None => return Err(SqltiseError::ColumnOutOfBounds),
				};

				if !validate_data(*sql_type, value) {
					add_validation_error(errors, value, *source, index);
				}
			}
			Column::FreeText { .. } => {}
		}
	}
	Ok(())
}

fn add_validation_error(errors: &mut Vec<ValidationError>, data: &str, column: usize, row: usize) {
	for error in errors.iter_mut() {
		if error.column == column && error.value == data {
			error.rows.push(row);
			return;
		}
	}

	let error = ValidationError {
		column,
		rows: vec![row],
		value: data.into(),
	};

	errors.push(error);
}

fn validate_data(sql_type: SqlType, data: &str) -> bool {
	match sql_type {
		SqlType::Int => is_valid_int(data),
		SqlType::Float => is_valid_float(data),
		SqlType::Date | SqlType::DateTime => is_valid_date(data),
		SqlType::NVarChar | SqlType::VarChar => check_varchar_errors(data),
		SqlType::Preformatted => true,
	}
}

fn is_valid_int(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return true;
	}
	value.parse::<i64>().is_ok()
}

fn is_valid_float(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return true;
	}
	value.parse::<f64>().is_ok()
}

fn is_valid_date(value: &str) -> bool {
	let value = value.trim();
	if is_null(value) {
		return true;
	}

	for format in &crate::DATETIME_FORMATS {
		let parsed = chrono::NaiveDateTime::parse_from_str(value.trim(), format);
		if parsed.is_ok() {
			return true;
		}
	}

	for format in &crate::DATE_FORMATS {
		let parsed = chrono::NaiveDate::parse_from_str(value.trim(), format);
		if parsed.is_ok() {
			return true;
		}
	}

	return false;
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
