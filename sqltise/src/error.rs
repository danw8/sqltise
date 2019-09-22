#[derive(Debug, PartialEq)]
pub enum SqltiseError {
    NoHeaders,
    InvalidHeaders,
    CsvParseError,
}