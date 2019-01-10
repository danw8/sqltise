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
}

#[derive(Serialize, Deserialize)]
pub enum ColumnType {
	Int,
	Float,
	Date,
	DateTime,
	VarChar,
}
