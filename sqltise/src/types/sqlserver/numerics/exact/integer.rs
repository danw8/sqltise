#![allow(unused)]
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };

pub struct Integer {
	value: i32,
}

impl Integer {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<i32>() {
			Ok(value) => Ok(Integer { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for Integer {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullInteger {
	value: Option<Integer>,
}

impl NullInteger {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullInteger { value: None });
		}
		let integer = Integer::new(value)?;
		Ok( NullInteger { value: Some(integer)})
	}
}

impl SqlType for NullInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(v) => v.to_sql(),
			None => "NULL".to_string(),
		}
	}
}


pub struct UnsignedInteger {
	value: u32,
}

impl UnsignedInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<u32>() {
			Ok(value) => Ok(UnsignedInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for UnsignedInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullUnsignedInteger {
	value: Option<UnsignedInteger>,
}

impl NullUnsignedInteger {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullUnsignedInteger {value: None});
		}
		let unsigned_integer = UnsignedInteger::new(value)?;
		Ok( NullUnsignedInteger { value: Some(unsigned_integer)})
	}
}

impl SqlType for NullUnsignedInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(v) => v.to_sql(),
			None => "NULL".to_string(),
		}
	}
}