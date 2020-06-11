endiannezz
==========
[![Build Status](https://travis-ci.org/Frago9876543210/endiannezz.svg?branch=master)](https://travis-ci.org/Frago9876543210/endiannezz)
[![Latest Version](https://img.shields.io/crates/v/endiannezz.svg)](https://crates.io/crates/endiannezz)
[![Documentation](https://docs.rs/endiannezz/badge.svg)](https://docs.rs/endiannezz/)

Zero dependencies library for I/O endianness on high-level

### Installing
```toml
[dependencies]
endiannezz = "0.3"
```
### Example
```rust
use std::io::Result;
use endiannezz::{NativeEndian, LittleEndian, BigEndian, ext::{EndianReader, EndianWriter}};

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
