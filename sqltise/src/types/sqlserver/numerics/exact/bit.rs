#![allow(unused)]
use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct Bit {
	value: bool,
}

impl Bit {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = match value.to_lowercase().trim() {
			"1" | "true" => true,
			"0" | "false" => false,
			_ => {
				return Err(SqlTypeError {
					message: "Could not parse bit value".to_string(),
				})
			}
		};
		Ok(Bit { value })
	}
}

impl SqlType for Bit {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		match self.value {
			true => 1.to_string(),
			false => 0.to_string(),
		}
	}
}

pub struct NullBit {
	value: Option<bool>,
}

impl NullBit {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		let value = match value.to_lowercase().trim() {
			"1" | "true" => Some(true),
			"0" | "false" => Some(false),
			"null" => None,
			_ => {
				return Err(SqlTypeError {
					message: "Could not parse nullable bit value".to_string(),
				})
			}
		};
		Ok(NullBit { value })
	}
}

impl SqlType for NullBit {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		match self.value {
			Some(v) => {
				if v {
					"1".to_string()
				} else {
					"0".to_string()
				}
			}
			None => "NULL".to_string(),
		}
	}
}
