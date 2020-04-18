endiannezz
==========

Zero dependencies library for I/O endianness on high-level

### Installing
```toml
[dependencies]
endiannezz = "0.1"
```
### Example
```rust
use std::io::Result;
use endiannezz::{BigEndian, EndianReader, EndianWriter, LittleEndian, NativeEndian};

fn main() -> Result<()> {
	let mut vec = Vec::new();

	vec.try_write::<LittleEndian, i32>(1)?;
	vec.try_write::<BigEndian, _>(2)?;
	vec.try_write::<NativeEndian, _>(3_u16)?;

	let mut slice = vec.as_slice();

	slice.try_read::<LittleEndian, i32>()?;
	let _num32: i32 = slice.try_read::<BigEndian, _>()?;
	let _num16: u16 = slice.try_read::<NativeEndian, _>()?;

	Ok(())
}
```
