#![allow(unused)]
use crate::types::{ SqlType, SqlTypeError, ErrorMessage };

pub struct Money {
	value: i64,
}

impl Money {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		let decimal_place = value.find('.');
		let (left, right) = if let Some(dp) = decimal_place {
			value.split_at(dp)
		} else {
			(value, "")
		};
		let mut left = left.replace('.', "");
		let mut right = right.replace('.', "");

		if right.len() < 4 {
			for _ in 0..(4-right.len()) {
				right.push('0');
			}
		} else if right.len() > 4 {
			return Err(SqlTypeError { message: "Could not parse money value".to_string() });
		}

		left.push_str(&right);
		return match left.trim().parse::<i64>() {
			Ok(value) => Ok(Money { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message})
			}
		}
	}
}

impl SqlType for Money {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		let value = self.value.to_string();
		let split: usize = value.len() - 4;
		
		let (left, right) = value.split_at(split);
		let mut value = left.to_string();
		value.push_str(".");
		value.push_str(right);
		value
	}
}

pub struct NullMoney {
	value: Option<Money>,
}

impl NullMoney {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullMoney {value: None});
		}
		let money = Money::new(value)?;
		Ok( NullMoney { value: Some(money)})
	}
}

impl SqlType for NullMoney {
	fn validate(&self) -> Result<(), ErrorMessage> {
		Ok(())
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(money) => money.to_sql(),
			None => "NULL".to_string()
		}
	}
}