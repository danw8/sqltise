pub mod error;
pub mod header;
pub mod statement;
pub mod validate;

const DATE_FORMATS: [&str; 6] = ["%F", "%D", "%v", "%Y-%m-%d", "%m/%d/%Y", r#"%m\%d\%Y"#];

const DATETIME_FORMATS: [&str; 20] = [
	"%+",
	"%c",
	"%m/%d/%Y %H:%M",
	"%m/%d/%Y %H:%M:%S",
	"%m/%d/%Y %H:%M:%S %p",
	"%m/%d/%Y %H:%M:%S%.f",
	"%m/%d/%Y %I:%M:%S %p",
	"%m/%d/%Y %I:%M:%S%.f",
	r#"%m\%d\%Y %H:%M"#,
	r#"%m\%d\%Y %H:%M:%S"#,
	r#"%m\%d\%Y %H:%M:%S %p"#,
	r#"%m\%d\%Y %H:%M:%S%.f"#,
	r#"%m\%d\%Y %I:%M:%S %p"#,
	r#"%m\%d\%Y %I:%M:%S%.f"#,
	"%Y-%m-%d %H:%M",
	"%Y-%m-%d %H:%M:%S",
	"%Y-%m-%d %H:%M:%S %p",
	"%Y-%m-%d %H:%M:%S%.f",
	"%Y-%m-%d %I:%M:%S %p",
	"%Y-%m-%d %I:%M:%S%.f",
];
