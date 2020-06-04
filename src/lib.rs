/*!
This crate provides the ability to encode and decode all primitive types into [different endianness]

# How it works?
Crate automatically implements [`Primitive`] trait for each primitive type.

This allows to write abstractions and call the appropriate method depending on
the byte order that you passed to the function template. [`Endian`] it's something
like a proxy to do it.

Macros create implementations for I/O endianness:
[`NativeEndian`], [`LittleEndian`] and [`BigEndian`]

All these types are enums, which means that you cannot create them, only pass to the template.

Now it's possible to have traits that expand [`Read`] and [`Write`] with new methods.

# Example
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

You can also use this syntax:
```rust
use endiannezz::{Endian, BigEndian, LittleEndian};

fn main() {
	let mut vec = Vec::new();
	BigEndian::write(1, &mut vec).unwrap();
	LittleEndian::write::<u16, _>(2, &mut vec).unwrap();
	assert_eq!(vec.as_slice(), &[0, 0, 0, 1, 2, 0])
}
```

[different endianness]: https://en.wikipedia.org/wiki/Endianness
[`Primitive`]: trait.Primitive.html
[`Endian`]: trait.Endian.html
[`NativeEndian`]: enum.NativeEndian.html
[`LittleEndian`]: enum.LittleEndian.html
[`BigEndian`]: enum.BigEndian.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
*/

use std::io::{Read, Result, Write};
use std::mem;

/// This trait is implemented for all primitive types that exist in rust,
/// and allows to read types from bytes or write them into bytes
//noinspection RsSelfConvention
pub trait Primitive: Sized + Copy {
	type Buf: AsRef<[u8]> + AsMut<[u8]> + Default;

	fn to_ne_bytes(self) -> Self::Buf;
	fn to_le_bytes(self) -> Self::Buf;
	fn to_be_bytes(self) -> Self::Buf;

	fn from_ne_bytes(bytes: Self::Buf) -> Self;
	fn from_le_bytes(bytes: Self::Buf) -> Self;
	fn from_be_bytes(bytes: Self::Buf) -> Self;
}

macro_rules! delegate {
	($ty:ty, [$($method:ident),* $(,)?], ($param:ident : $param_ty:ty) -> $ret:ty) => {
		delegate!(@inner $ty, [$($method),*], $param, $param_ty, $ret);
	};
	(@inner $ty:ty, [$($method:ident),*], $param:ident, $param_ty:ty, $ret:ty) => {
		$(
			#[inline]
			fn $method ($param: $param_ty) -> $ret { <$ty>::$method($param) }
		)*
	};
}

macro_rules! impl_primitives {
	($($ty:ty),* $(,)?) => {
		$(
			impl Primitive for $ty {
				type Buf = [u8; mem::size_of::<$ty>()];

				delegate!($ty, [
					to_ne_bytes,
					to_le_bytes,
					to_be_bytes,
				], (self: Self) -> Self::Buf);

				delegate!($ty, [
					from_ne_bytes,
					from_le_bytes,
					from_be_bytes,
				], (bytes: Self::Buf) -> Self);
			}
		)*
	};
}

impl_primitives![
	i8,    u8,
	i16,   u16,
	i32,   u32,
	f32,   f64,
	i64,   u64,
	i128,  u128,
	isize, usize,
];

/// Proxy for reading and writing primitive types
pub trait Endian {
	fn write<T: Primitive, W: Write>(primitive: T, w: W) -> Result<()>;

	fn read<T: Primitive, R: Read>(r: R) -> Result<T>;
}

macro_rules! impl_endianness {
	($($endian:ident $write:ident $read:ident,)*) => {
		$(
			pub enum $endian {}

			impl Endian for $endian {
				#[inline]
				fn write<T: Primitive, W: Write>(primitive: T, mut w: W) -> Result<()> {
					w.write_all(primitive.$write().as_ref())
				}

				#[inline]
				fn read<T: Primitive, R: Read>(mut r: R) -> Result<T> {
					let mut buf = T::Buf::default();
					r.read_exact(&mut buf.as_mut())?;
					Ok(T::$read(buf))
				}
			}
		)*
	};
}

impl_endianness![
	NativeEndian to_ne_bytes from_ne_bytes,
	LittleEndian to_le_bytes from_le_bytes,
	BigEndian    to_be_bytes from_be_bytes,
];

/// Allows to read primitive types with differents representation of bytes
pub trait EndianReader: Read {
	fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T>;
}

impl<R: Read + ?Sized> EndianReader for R {
	#[inline]
	fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T> {
		E::read(self)
	}
}

/// Allows to write primitive types with differents representation of bytes
pub trait EndianWriter: Write {
	fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()>;
}

impl<W: Write + ?Sized> EndianWriter for W {
	#[inline]
	fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()> {
		E::write(primitive, self)
	}
}
