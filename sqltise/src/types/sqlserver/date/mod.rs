pub mod date;
pub mod datetime;
pub mod datetime2;
pub mod datetime_offset;
pub mod small_datetime;
pub mod time;

pub use date::*;
pub use datetime::*;
pub use datetime2::*;
pub use datetime_offset::*;
pub use small_datetime::*;
pub use time::*;

const TIME_FORMATS: [&str; 6] = [
	"%H:%M",
	"%H:%M:%S",
	"%H:%M:%S %p",
	"%H:%M:%S%.f",
	"%I:%M:%S %p",
	"%I:%M:%S%.f",
];

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

const DATETIME_OFFSET_FORMATS: [&str; 18] = [
	"%m/%d/%Y %H:%M%z",
	"%m/%d/%Y %H:%M:%S%z",
	"%m/%d/%Y %H:%M:%S %p%z",
	"%m/%d/%Y %H:%M:%S%.f%z",
	"%m/%d/%Y %I:%M:%S %p%z",
	"%m/%d/%Y %I:%M:%S%.f%z",
	r#"%m\%d\%Y %H:%M%z"#,
	r#"%m\%d\%Y %H:%M:%S%z"#,
	r#"%m\%d\%Y %H:%M:%S %p%z"#,
	r#"%m\%d\%Y %H:%M:%S%.f%z"#,
	r#"%m\%d\%Y %I:%M:%S %p%z"#,
	r#"%m\%d\%Y %I:%M:%S%.f%z"#,
	"%Y-%m-%d %H:%M%z",
	"%Y-%m-%d %H:%M:%S%z",
	"%Y-%m-%d %H:%M:%S %p%z",
	"%Y-%m-%d %H:%M:%S%.f%z",
	"%Y-%m-%d %I:%M:%S %p%z",
	"%Y-%m-%d %I:%M:%S%.f%z",
];
