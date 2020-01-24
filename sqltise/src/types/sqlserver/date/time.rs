use super::TIME_FORMATS;
use crate::types::{ErrorMessage, SqlType, SqlTypeError};
use chrono::naive::NaiveTime;

pub struct Time {
	value: NaiveTime,
}

impl Time {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = value.trim();
		for format in &TIME_FORMATS {
			let parsed = NaiveTime::parse_from_str(value, format);
			if parsed.is_ok() {
				let value = parsed.unwrap();
				let time = Time { value };
				return Ok(time);
			}
		}
		return Err(SqlTypeError {
			message: "Couldn't parse as Time".to_string(),
		});
	}
}

impl SqlType for Time {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.format("%H:%M:%S%.f");
		format!("'{}'", value)
	}
}

pub struct NullTime {
	value: Option<Time>,
}

impl NullTime {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullTime { value: None });
		}
		let time = Time::new(value)?;
		Ok(NullTime { value: Some(time) })
	}
}

impl SqlType for NullTime {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(date) => date.validate(),
			None => Ok(()),
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(date) => date.to_sql(),
			None => "NULL".to_string(),
		}
	}
}
