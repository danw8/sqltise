use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct Character {
	byte_length: usize,
	value: String,
}

impl Character {
	pub fn new(value: &str, byte_length: usize) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Err(SqlTypeError {
				message: "Character cannot be null".to_string(),
			});
		}
		Ok(Character {
			byte_length,
			value: value.to_string(),
		})
	}
}

impl SqlType for Character {
	fn validate(&self) -> Result<(), ErrorMessage> {
		if self.value.bytes().len() > self.byte_length {
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

pub struct NullCharacter {
	value: Option<Character>,
}

impl NullCharacter {
	pub fn new(value: &str, byte_length: usize) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullCharacter { value: None });
		}
		let character = Character::new(value, byte_length)?;
		Ok(NullCharacter {
			value: Some(character),
		})
	}
}

impl SqlType for NullCharacter {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(character) => character.validate(),
			None => Ok(()),
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(character) => character.to_sql(),
			None => "NULL".to_string(),
		}
	}
}
