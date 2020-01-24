use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct Float {
	value: f64,
}

impl Float {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		return match value.trim().parse::<f64>() {
			Ok(value) => Ok(Float { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message })
			}
		};
	}
}

impl SqlType for Float {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.to_string()
	}
}

pub struct NullFloat {
	value: Option<Float>,
}

impl NullFloat {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullFloat { value: None });
		}
		let float = Float::new(value)?;
		Ok(NullFloat { value: Some(float) })
	}
}

impl SqlType for NullFloat {
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
