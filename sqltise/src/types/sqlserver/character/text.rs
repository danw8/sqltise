use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct Text {
	value: String,
}

impl Text {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Err(SqlTypeError {
				message: "Text cannot be null".to_string(),
			});
		}
		Ok(Text {
			value: value.to_string(),
		})
	}
}

impl SqlType for Text {
	fn validate(&self) -> Result<(), ErrorMessage> {
		if self.value.bytes().len() > 2 ^ 31 - 1 {
			return Err(ErrorMessage(
				"Value is longer than expected length".to_string(),
			));
		}
		if !self.value.is_ascii() {
			return Err(ErrorMessage(
				"Value can only contains ascii characters".to_string(),
			));
		}
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.replace("'", "''");
		format!("'{}'", value)
	}
}

pub struct NullText {
	value: Option<Text>,
}

impl NullText {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullText { value: None });
		}
		let text = Text::new(value)?;
		Ok(NullText { value: Some(text) })
	}
}

impl SqlType for NullText {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(text) => text.validate(),
			None => Ok(()),
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(text) => text.to_sql(),
			None => "NULL".to_string(),
		}
	}
}
