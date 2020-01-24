use super::DATETIME_FORMATS;
use crate::types::{ErrorMessage, SqlType, SqlTypeError};
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};

pub struct SmallDateTime {
	value: NaiveDateTime,
}

impl SmallDateTime {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = value.trim();
		for format in &DATETIME_FORMATS {
			let parsed = NaiveDateTime::parse_from_str(value, format);
			if parsed.is_ok() {
				let value = parsed.unwrap();
				let small_date_time = SmallDateTime { value };
				return Ok(small_date_time);
			}
		}
		return Err(SqlTypeError {
			message: "Couldn't parse as DateTime".to_string(),
		});
	}
}

impl SqlType for SmallDateTime {
	fn validate(&self) -> Result<(), ErrorMessage> {
		if self.value
			< NaiveDateTime::new(
				NaiveDate::from_ymd(1900, 01, 01),
				NaiveTime::from_hms(0, 0, 0),
			) {
			return Err(ErrorMessage(
				"Value is less than '1900-01-01 00:00:00'".to_string(),
			));
		}
		if self.value
			>= NaiveDateTime::new(
				NaiveDate::from_ymd(2079, 06, 06),
				NaiveTime::from_hms(23, 59, 59),
			) {
			return Err(ErrorMessage(
				"Value is greater than '2079-06-06 23:59:59'".to_string(),
			));
		}
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.format("%Y-%m-%d %H:%M:%S");
		format!("'{}'", value)
	}
}

pub struct NullSmallDateTime {
	value: Option<SmallDateTime>,
}

impl NullSmallDateTime {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullSmallDateTime { value: None });
		}
		let small_date_time = SmallDateTime::new(value)?;
		Ok(NullSmallDateTime {
			value: Some(small_date_time),
		})
	}
}

impl SqlType for NullSmallDateTime {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(small_date_time) => small_date_time.validate(),
			None => Ok(()),
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(small_date_time) => small_date_time.to_sql(),
			None => "NULL".to_string(),
		}
	}
}
