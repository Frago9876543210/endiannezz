use std::io::{Read, Result, Write};

use crate::{Endian, Io, Primitive};

pub trait HackedPrimitive: Primitive {
	#[cfg_attr(feature = "inline_primitives", inline)]
	fn write_hacked<E: Endian, W: Write>(self, w: W) -> Result<()> {
		E::write(self, w)
	}

	#[cfg_attr(feature = "inline_primitives", inline)]
	fn read_hacked<E: Endian, R: Read>(r: R) -> Result<Self> {
		E::read(r)
	}
}

impl<T: Primitive> HackedPrimitive for T {}

pub trait HackedIo: Io {
	#[cfg_attr(feature = "inline_io", inline(always))]
	fn write_hacked<E: Endian, W: Write>(&self, w: W) -> Result<()> {
		Io::write(self, w)
	}

	#[cfg_attr(feature = "inline_io", inline(always))]
	fn read_hacked<E: Endian, R: Read>(r: R) -> Result<Self> {
		Io::read(r)
	}
}

impl<T: Io> HackedIo for T {}
