use std::io::{Read, Result, Write};

use crate::{Endian, Primitive};

/// Allows to write primitive types with differents representation of bytes
pub trait EndianWriter: Write {
	#[inline]
	fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()> {
		E::write(primitive, self)
	}
}

impl<W: Write + ?Sized> EndianWriter for W {}

/// Allows to read primitive types with differents representation of bytes
pub trait EndianReader: Read {
	#[inline]
	fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T> {
		E::read(self)
	}
}

impl<R: Read + ?Sized> EndianReader for R {}
