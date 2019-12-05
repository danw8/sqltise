#![allow(unused)]
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };

pub struct Numeric {
	precision: usize,
	scale: usize,
	value: String,
}

impl Numeric {
	fn new(value: &str, precision: usize, scale: usize) -> Self {
		Numeric {
			precision,
			scale,
			value: value.to_string(),
		}
	}
}

impl SqlType for Numeric {
	fn validate(&self) -> Result<(), ErrorMessage> {
		let valid = self.value.chars()
			.all(|c| c.is_numeric() || c == '.');
		
		if !valid {
			return Err(ErrorMessage("Numerics can only consist of 0-9 and '.'".to_string()));
		}

		let decimal_place = self.value.find('.');
		let (left, right) = if let Some(dp) = decimal_place {
			self.value.split_at(dp)
		} else {
			(self.value.as_str(), "")
		};
		let left = left.replace('.', "");
		let right = right.replace('.', "");

		if left.len() > self.precision {
			return Err(ErrorMessage("Value does not meet the percision requirement".to_string()));
		}

		if right.len() > self.scale {
			return Err(ErrorMessage("Value does not meet the scale requirement".to_string()));
		}

		Ok(())
	}

	fn to_sql(&self) -> String {
		self.value.clone()
	}
}

pub struct NullNumeric {
	value: Option<Numeric>
}

impl NullNumeric {
	fn new(value: &str, precision: usize, scale: usize) -> Self {
		if value.to_lowercase().trim() == "null" {
			return NullNumeric { value: None };
		}
		NullNumeric { value: Some(Numeric::new(value, precision, scale))}
	}
}

impl SqlType for NullNumeric {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(numeric) =>  {
				numeric.validate()
			},
			None => {
				Ok(())
			}
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(numeric) => numeric.to_sql(),
			None => "NULL".to_string()
		}
	}
}