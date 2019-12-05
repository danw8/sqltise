use chrono::prelude::*;
use chrono::DateTime;
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };
use super::DATETIME_OFFSET_FORMATS;


pub struct DateTimeOffset {
	value: DateTime<FixedOffset>,
}

impl DateTimeOffset {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = value.trim();
		for format in &DATETIME_OFFSET_FORMATS {
			let parsed = DateTime::parse_from_str(value, format);
			if parsed.is_ok() {
				let value = parsed.unwrap();
				let date_time_2 = DateTimeOffset {
					value
				};
				return Ok(date_time_2);
			}
		}
		return Err(SqlTypeError { message: "Couldn't parse as DateTime".to_string()})
	}
}

impl SqlType for DateTimeOffset {
	fn validate(&self) -> Result<(), ErrorMessage> {
		let year_one = DateTime::<FixedOffset>::from_utc(NaiveDateTime::new(NaiveDate::from_ymd(0001, 01, 01), NaiveTime::from_hms(0, 0, 0)), FixedOffset::west(0));
		let year_nine_nine_nine = DateTime::<FixedOffset>::from_utc(NaiveDateTime::new(NaiveDate::from_ymd(9999, 12, 31), NaiveTime::from_hms_milli(23, 59, 59, 9999999)), FixedOffset::west(0));
		if self.value < year_one {
			return Err(ErrorMessage("Value is less than '0001-01-01 00:00:00'".to_string()));
		}
		if self.value >= year_nine_nine_nine {
			return Err(ErrorMessage("Value is greater than '9999-12-31 23:59:59.997'".to_string()));
		}
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.format("%Y-%m-%d %H:%M:%S%.f");
		format!("'{}'", value)
	}
}

pub struct NullDateTimeOffset {
	value: Option<DateTimeOffset>
}

impl NullDateTimeOffset {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullDateTimeOffset { value: None } );
		}
		let date_time_offset = DateTimeOffset::new(value)?;
		Ok( NullDateTimeOffset { value: Some(date_time_offset) })
	}
}

impl SqlType for NullDateTimeOffset {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(date_time_offset) =>  {
				date_time_offset.validate()
			},
			None => {
				Ok(())
			}
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(date_time_2) => date_time_2.to_sql(),
			None => "NULL".to_string()
		}
	}
}