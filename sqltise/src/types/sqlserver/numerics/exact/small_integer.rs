#![allow(unused)]
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };

pub struct SmallInteger {
	value: i16,
}

impl SmallInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<i16>() {
			Ok(value) => Ok(SmallInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for SmallInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullSmallInteger {
	value: Option<SmallInteger>,
}

impl NullSmallInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullSmallInteger { value: None } );
		}
		let small_integer = SmallInteger::new(value)?;
		Ok( NullSmallInteger { value: Some(small_integer) })
	}
}

impl SqlType for NullSmallInteger {
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


pub struct UnsignedSmallInteger {
	value: u16,
}

impl UnsignedSmallInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<u16>() {
			Ok(value) => Ok(UnsignedSmallInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for UnsignedSmallInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullUnsignedSmallInteger {
	value: Option<UnsignedSmallInteger>,
}

impl NullUnsignedSmallInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullUnsignedSmallInteger { value: None } );
		}
		let unsigned_small_integer = UnsignedSmallInteger::new(value)?;
		Ok( NullUnsignedSmallInteger { value: Some(unsigned_small_integer) })
	}
}

impl SqlType for NullUnsignedSmallInteger {
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