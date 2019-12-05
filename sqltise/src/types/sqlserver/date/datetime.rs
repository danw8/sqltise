use chrono::naive::{NaiveDateTime, NaiveDate, NaiveTime};
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };
use super::DATETIME_FORMATS;

pub struct DateTime {
	value: NaiveDateTime,
}

impl DateTime {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = value.trim();
		for format in &DATETIME_FORMATS {
			let parsed = NaiveDateTime::parse_from_str(value, format);
			if parsed.is_ok() {
				let value = parsed.unwrap();
				let date_time = DateTime {
					value
				};
				return Ok(date_time);
			}
		}
		return Err(SqlTypeError { message: "Couldn't parse as DateTime".to_string()})
	}
}

impl SqlType for DateTime {
	fn validate(&self) -> Result<(), ErrorMessage> {
		if self.value < NaiveDateTime::new(NaiveDate::from_ymd(1753, 01, 01), NaiveTime::from_hms(0, 0, 0)) {
			return Err(ErrorMessage("Value is less than '1753-01-01 00:00:00'".to_string()));
		}
		if self.value >= NaiveDateTime::new(NaiveDate::from_ymd(9999, 12, 31), NaiveTime::from_hms_milli(23, 59, 59, 997)) {
			return Err(ErrorMessage("Value is greater than '9999-12-31 23:59:59.997'".to_string()));
		}
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.format("%Y-%m-%d %H:%M:%S%.f");
		format!("'{}'", value)
	}
}

pub struct NullDateTime {
	value: Option<DateTime>
}

impl NullDateTime {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullDateTime { value: None } );
		}
		let date_time = DateTime::new(value)?;
		Ok( NullDateTime { value: Some(date_time) })
	}
}

impl SqlType for NullDateTime {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(date_time) =>  {
				date_time.validate()
			},
			None => {
				Ok(())
			}
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(date_time) => date_time.to_sql(),
			None => "NULL".to_string()
		}
	}
}