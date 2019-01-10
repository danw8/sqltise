#[derive(Serialize)]
pub struct CsvHeaders {
	pub columns: Vec<ColumnHeader>,
}

#[derive(Serialize)]
pub struct ColumnHeader {
	pub name: String,
	pub index: usize,
}

#[derive(Serialize, Deserialize)]
pub struct StatementSelections {
	pub value: Vec<StatementSelection>
}

#[derive(Serialize, Deserialize)]
pub struct StatementSelection {
	pub id: usize,
	pub r#type: StatementType,
	pub name: String,
	pub table: String,
}

#[derive(Serialize, Deserialize)]
pub enum StatementType {
	Insert,
	Update
}

#[derive(Serialize, Deserialize)]
pub struct ColumnSelections {
	pub value: Vec<ColumnSelection>
}

#[derive(Serialize, Deserialize)]
pub struct ColumnSelection {
	pub column: usize,
	pub statement_id: usize,
	pub name: Option<String>,
	pub r#type: ColumnType,
	pub use_source: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ColumnType {
	Int,
	Float,
	Date,
	DateTime,
	VarChar,
}

#[derive(Serialize, Deserialize)]
pub struct CsvErrors {
	pub value: Vec<CsvError>
}

#[derive(Serialize, Deserialize)]
pub struct CsvError {
	pub statement_id: usize,
	pub column_id: usize,
	pub r#type: ColumnType,
	pub error_text: String,
	pub rows: Vec<usize>,
}


#[derive(Serialize, Deserialize)]
pub struct ParseError {
	pub error: String,
	pub kind: String,
}