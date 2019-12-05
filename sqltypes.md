# Sql Server

## Data Types

### Exact Numerics

* `bigint` 8 Bytes, rusts `i64`
* `bigint unsigned` 8 Bytes, rusts `u64`
* `int` 4 Bytes, rusts `i32`
* `int unsigned` 4 Bytes, rusts `u32`
* `smallint` 2 Bytes, rusts `i16`
* `smallint unsigned` 2 Bytes, rusts `u16`
* `tinyint` 1 Bytes, rusts `i8`
* `tinyint unsigned` 1 Bytes, rusts `u8`
* `decimal(p, s)` ??
* `numeric(p, s)` ??
* `bit` rusts 1i8, 0i8 or None


* `money` 8 bytes, rusts `i64` without decimal place
* `smallmoney` 4 bytes, rusts `i32` without decimal place

### Approximate numerics

* `float` 8 bytes, rusts `f64`
* `real` 4 bytes, rusts `f32`

### Dates and Time

\* does not support Informatica

* `date`, rust chrono NaiveDate, 0001-01-01 through 9999-12-31
* `datetime2`, rust chrono NaiveDateTime, 0001-01-01 through 9999-12-31, time range 00:00:00 through 23:59:59.9999999
* `datetime`, rust chrono NaiveDateTime, 1753-01-01 through 9999-12-31, 00:00:00 through 23:59:59.997
* `datetimeoffset`, rust chrono DateTime, 0001-01-01 through 9999-12-31, time range 00:00:00 through 23:59:59.9999999
* `smalldatetime`, rust chrono NaiveDateTime, 1900-01-01 through 2079-06-06, time range 00:00:00 through 23:59:59
* `time`, rust chrono NaiveTime, 00:00:00.0000000 through 23:59:59.9999999

### Character strings

* `char(n)`, rusts String.is_ascii() with `n` bytes
* `varchar(n|max)`, rusts String.is_ascii() with `n` bytes or 2^31-1(1,073,741,823) bytes
* `text` rust String.is_ascii() with up to 2^31-1(1,073,741,823) bytes

### Unicode character strings

* `nchar`, rust String with `n` bytes
* `nvarchar`, rusts String with `n` bytes or 2^31-1(1,073,741,823) bytes
* `ntext` rust String with 2^31-1(1,073,741,823) bytes

### Unsupported Types

\* Not sure if sqltise should support these if if it would ever even make sense to.

* `cursor`
* `rowversion`
* `hierarchyid`
* `uniqueidentifier`
* `sql_variant`
* `xml`
* `Spatial Geometry Types`
* `Spatial Geogrphy Types`
* `table`