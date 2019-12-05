#![allow(unused)]
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };

pub struct BigInteger {
	value: i64,
}

impl BigInteger {
	fn new(source: &str) -> Result<Self, SqlTypeError> {
		return match source.trim().parse::<i64>() {
			Ok(value) => Ok(BigInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for BigInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullBigInteger {
	value: Option<BigInteger>,
}

impl NullBigInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullBigInteger { value: None } );
		}
		let big_integer = BigInteger::new(value)?;
		Ok( NullBigInteger { value: Some(big_integer) })
	}
}

impl SqlType for NullBigInteger {
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

pub struct UnsignedBigInteger {
	value: u64,
}

impl UnsignedBigInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<u64>() {
			Ok(value) => Ok(UnsignedBigInteger { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for UnsignedBigInteger {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullUnsignedBigInteger {
	value: Option<UnsignedBigInteger>,
}

impl NullUnsignedBigInteger {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok( NullUnsignedBigInteger { value: None } );
		}
		let unsigned_big_integer = UnsignedBigInteger::new(value)?;
		Ok( NullUnsignedBigInteger { value: Some(unsigned_big_integer) })
	}
}

impl SqlType for NullUnsignedBigInteger {
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