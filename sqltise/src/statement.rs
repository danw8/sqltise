pub struct Statement {
	pub name: String,
	pub info: StatementInfo,
}

pub enum StatementInfo {
	Insert {
		table: String,
		columns: Vec<Column>,
	},
	Update {
		table: String,
		columns: Vec<Column>,
		conditions: Vec<Condition>,
	},
	Custom {
		value: String,
	},
}

pub enum Column {
	Csv {
		source: usize,
		sql_type: SqlType,
		destination: Option<String>,
	},
	FreeText {
		data: String,
		destination: String,
	},
}

#[derive(Copy, Clone)]
pub enum SqlType {
	Int,
	Float,
	Date,
	DateTime,
	VarChar,
	NVarChar,
	Preformatted,
}

pub struct Condition {
	pub column: String,
	pub source: usize,
	pub sql_type: SqlType,
}
