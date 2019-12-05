use chrono::naive::NaiveDate;
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };
use super::DATE_FORMATS;

pub struct Date {
	value: NaiveDate,
}

impl Date {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = value.trim();
		for format in &DATE_FORMATS {
			let parsed = NaiveDate::parse_from_str(value, format);
			if parsed.is_ok() {
				let value = parsed.unwrap();
				let date =  Date {
					value
				};
				return Ok(date);
			}
		}
		return Err(SqlTypeError { message: "Couldn't parse as Date".to_string() })
	}
}

impl SqlType for Date {
	fn validate(&self) -> Result<(), ErrorMessage> {
		if self.value < NaiveDate::from_ymd(0001, 01, 01) {
			return Err(ErrorMessage("Value is less than '0001-01-01".to_string()));
		}
		if self.value > NaiveDate::from_ymd(9999,12,31) {
			return Err(ErrorMessage("Value is greater than '9999-12-31'".to_string()));
		}
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.format("%Y-%m-%d");
		format!("'{}'", value)
	}
}

pub struct NullDate {
	value: Option<Date>
}

impl NullDate {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullDate { value: None } );
		}
		let date = Date::new(value)?;
		Ok( NullDate { value: Some(date) })
	}
}

impl SqlType for NullDate {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(date) =>  {
				date.validate()
			},
			None => {
				Ok(())
			}
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(date) => date.to_sql(),
			None => "NULL".to_string()
		}
	}
}