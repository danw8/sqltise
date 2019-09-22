use crate::error::SqltiseError;

#[derive(PartialEq, Debug)]
pub struct Header {
	pub index: usize,
	pub value: String,
}

pub fn get_headers(csv_data: &str) -> Result<Vec<Header>, SqltiseError> {
	let mut reader = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_reader(csv_data.as_bytes());

	let mut headers = Vec::new();

	let header_values = reader
		.headers()
		.expect("This should never fail because Strings slices are always valid UTF-8.");

	if header_values.len() == 0 {
		return Err(SqltiseError::NoHeaders);
	}

	for (index, header) in header_values.iter().enumerate() {
		let header = Header {
			index,
			value: header.trim().into(),
		};
		headers.push(header);
	}

	Ok(headers)
}

#[cfg(test)]
mod test {

	use crate::error::SqltiseError;
	use crate::header::*;

	#[test]
	fn test_get_headers_finds_headers() {
		let header_data = r#"One, Two, Three
1, 2, 3
4, 5, 6
"#;

		let result = get_headers(header_data);

		assert!(result.is_ok());
		let result = result.unwrap();
		assert_eq!(result.len(), 3);
		assert_eq!(
			result[0],
			Header {
				index: 0,
				value: "One".into()
			}
		);
		assert_eq!(
			result[1],
			Header {
				index: 1,
				value: "Two".into()
			}
		);
		assert_eq!(
			result[2],
			Header {
				index: 2,
				value: "Three".into()
			}
		);
	}

	#[test]
	fn test_get_header_fails_empty() {
		let header_data = "";

		let result = get_headers(header_data);
		assert!(result.is_err());
		match result {
			Ok(_) => unreachable!(),
			Err(e) => assert_eq!(e, SqltiseError::NoHeaders),
		}
	}
}
