#![allow(unused)]
use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct Real {
	value: f32,
}

impl Real {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<f32>() {
			Ok(value) => Ok(Real { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message })
			}
		};
	}
}

impl SqlType for Real {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullReal {
	value: Option<Real>,
}

impl NullReal {
	fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullReal { value: None });
		}
		let real = Real::new(value)?;
		Ok(NullReal { value: Some(real) })
	}
}

impl SqlType for NullReal {
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
