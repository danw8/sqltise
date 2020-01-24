use super::DATETIME_FORMATS;
use crate::types::{ErrorMessage, SqlType, SqlTypeError};
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};

pub struct DateTime2 {
	value: NaiveDateTime,
}

impl DateTime2 {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = value.trim();
		for format in &DATETIME_FORMATS {
			let parsed = NaiveDateTime::parse_from_str(value, format);
			if parsed.is_ok() {
				let value = parsed.unwrap();
				let date_time_2 = DateTime2 { value };
				return Ok(date_time_2);
			}
		}
		return Err(SqlTypeError {
			message: "Couldn't parse as DateTime".to_string(),
		});
	}
}

impl SqlType for DateTime2 {
	fn validate(&self) -> Result<(), ErrorMessage> {
		if self.value
			< NaiveDateTime::new(
				NaiveDate::from_ymd(0001, 01, 01),
				NaiveTime::from_hms(0, 0, 0),
			) {
			return Err(ErrorMessage(
				"Value is less than '0001-01-01 00:00:00'".to_string(),
			));
		}
		if self.value
			>= NaiveDateTime::new(
				NaiveDate::from_ymd(9999, 12, 31),
				NaiveTime::from_hms_milli(23, 59, 59, 997),
			) {
			return Err(ErrorMessage(
				"Value is greater than '9999-12-31 23:59:59.997'".to_string(),
			));
		}
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.format("%Y-%m-%d %H:%M:%S%.f");
		format!("'{}'", value)
	}
}

pub struct NullDateTime2 {
	value: Option<DateTime2>,
}

impl NullDateTime2 {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullDateTime2 { value: None });
		}
		let date_time_2 = DateTime2::new(value)?;
		Ok(NullDateTime2 {
			value: Some(date_time_2),
		})
	}
}

impl SqlType for NullDateTime2 {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(date_time_2) => date_time_2.validate(),
			None => Ok(()),
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(date_time_2) => date_time_2.to_sql(),
			None => "NULL".to_string(),
		}
	}
}
