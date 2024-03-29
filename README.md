endiannezz
==========
[![Build Status](https://github.com/Frago9876543210/endiannezz/workflows/CI/badge.svg)](https://github.com/Frago9876543210/endiannezz/actions)
[![Latest Version](https://img.shields.io/crates/v/endiannezz.svg)](https://crates.io/crates/endiannezz)
[![Documentation](https://docs.rs/endiannezz/badge.svg)](https://docs.rs/endiannezz/)

Zero dependencies library for I/O endianness on high-level

### Installing
```toml
[dependencies]
endiannezz = "0.6"
```
### Using `#[derive(Io)]`
```rust
use endiannezz::Io;
use std::io::Result;

#[derive(Io)]
#[endian(big)]
struct ParseMe {
    works: bool,
    data: u32,
    #[endian(little)]
    extra: i16,
}

fn main() -> Result<()> {
    let s1 = ParseMe {
        works: true,
        data: 10,
        extra: 20,
    };

    //writing struct as bytes into vec
    let mut vec = Vec::new();
    s1.write(&mut vec)?;

    let mut slice = vec.as_slice();
    #[rustfmt::skip]
    assert_eq!(slice, &[
        1, //bool as byte
        0, 0, 0, 10, //u32 in big-endian (because big-endian is set on top place struct as default)
        20, 0, //i16 in little-endian (overriding default)
    ]);

    //reading struct from bytes
    let _s2 = ParseMe::read(&mut slice)?;

    Ok(())
}
```

### Simple example
```rust
use endiannezz::ext::{EndianReader, EndianWriter};
use endiannezz::{BigEndian, LittleEndian, NativeEndian};
use std::io::Result;

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
