pub use crate::statement::*;
pub use crate::error::*;
use csv::StringRecord;

pub struct ValidationError {
    pub column: usize,
    pub rows: Vec<usize>,
    pub value: String,
}

pub fn validate(data: &str, statements: Vec<Statement>) -> Result<Vec<ValidationError>, SqltiseError> {
    let mut errors = vec!();

    let mut reader = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_reader(data.as_bytes());

    for (index, row) in reader.records().enumerate() {
        let record = row.map_err(|_| SqltiseError::CsvParseError)?;
        if record.iter().all(|r| r.trim().is_empty()) { continue; }
        for statement in &statements {
            check_row_for_errors(statement, index, &record, &mut errors);
        }
    }

    Ok(errors)
}

fn check_row_for_errors(statement: &Statement, index: usize, row: &StringRecord, mut errors: &mut Vec<ValidationError> ) {
    match statement {
        _ => {}
    }
}