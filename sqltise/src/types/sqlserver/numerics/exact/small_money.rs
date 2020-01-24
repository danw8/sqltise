#![allow(unused)]
use crate::types::{ErrorMessage, SqlType, SqlTypeError};

pub struct SmallMoney {
	value: i32,
}

impl SmallMoney {
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
			for _ in 0..(4 - right.len()) {
				right.push('0');
			}
		} else if right.len() > 4 {
			return Err(SqlTypeError {
				message: "Could not parse money value".to_string(),
			});
		}

		left.push_str(&right);
		return match left.trim().parse::<i32>() {
			Ok(value) => Ok(SmallMoney { value }),
			Err(e) => {
				let message = format!("parsing failed: {}", e);
				Err(SqlTypeError { message })
			}
		};
	}
}

impl SqlType for SmallMoney {
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

pub struct NullSmallMoney {
	value: Option<SmallMoney>,
}

impl NullSmallMoney {
	pub fn new(value: &str) -> Result<Self, SqlTypeError> {
		if value.to_lowercase().trim() == "null" {
			return Ok(NullSmallMoney { value: None });
		}
		let small_money = SmallMoney::new(value)?;
		Ok(NullSmallMoney {
			value: Some(small_money),
		})
	}
}

impl SqlType for NullSmallMoney {
	fn validate(&self) -> Result<(), ErrorMessage> {
		match &self.value {
			Some(small_money) => small_money.validate(),
			None => Ok(()),
		}
	}

	fn to_sql(&self) -> String {
		match &self.value {
			Some(small_money) => small_money.to_sql(),
			None => "NULL".to_string(),
		}
	}
}
