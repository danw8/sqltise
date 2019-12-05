pub mod sqlserver;

pub struct SqlTypeError {
	pub message: String,
}

pub struct ErrorMessage(String);

pub trait SqlType where Self: std::marker::Sized {
	fn validate(&self) -> Result<(), ErrorMessage>;
	fn to_sql(&self) -> String;
}