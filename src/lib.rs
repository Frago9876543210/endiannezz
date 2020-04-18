use std::io::{Read, Result, Write};
use std::mem;

//noinspection RsSelfConvention
pub trait Primitive: Sized {
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
	i64,   u64,
	i128,  u128,
	isize, usize,
];

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

pub trait EndianReader: Read {
	fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T>;
}

impl<R: Read + ?Sized> EndianReader for R {
	#[inline]
	fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T> {
		E::read(self)
	}
}

pub trait EndianWriter: Write {
	fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()>;
}

impl<W: Write + ?Sized> EndianWriter for W {
	#[inline]
	fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()> {
		E::write(primitive, self)
	}
}
