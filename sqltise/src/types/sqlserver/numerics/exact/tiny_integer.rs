#![allow(unused)]
use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct TinyInteger {
	value: i8,
}

impl TinyInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<i8>() {
			Ok(value) => Ok(TinyInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message })
			}
		};
	}
}

impl SqlType for TinyInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullTinyInteger {
	value: Option<TinyInteger>,
}

impl NullTinyInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullTinyInteger { value: None });
		}
		let tiny_integer = TinyInteger::new(value)?;
		Ok(NullTinyInteger {
			value: Some(tiny_integer),
		})
	}
}

impl SqlType for NullTinyInteger {
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

pub struct UnsignedTinyInteger {
	value: u8,
}

impl UnsignedTinyInteger {
	fn from_str(source: &str) -> Result<Self, SqlTypeError> {
		return match source.trim().parse::<u8>() {
			Ok(value) => Ok(UnsignedTinyInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message })
			}
		};
	}
}

impl SqlType for UnsignedTinyInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullUnsignedTinyInteger {
	value: Option<u8>,
}

impl NullUnsignedTinyInteger {
	fn from_str(source: &str) -> Result<Self, SqlTypeError> {
		return match source.trim().parse::<u8>() {
			Ok(value) => Ok(NullUnsignedTinyInteger { value: Some(value) }),
			Err(e) => {
				if source.to_lowercase().trim() == "null" {
					Ok(NullUnsignedTinyInteger { value: None })
				} else {
					let message = format!("parsing failed: {}", e);
					Err(SqlTypeError { message })
				}
			}
		};
	}
}

impl SqlType for NullUnsignedTinyInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		match self.value {
			Some(v) => v.to_string(),
			None => "NULL".to_string(),
		}
	}
}
