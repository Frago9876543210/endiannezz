use std::io::{Read, Result, Write};

use crate::{Io, Endian, Primitive};

pub trait HackedPrimitive: Primitive {
	fn write_hacked<E: Endian, W: Write>(self, w: W) -> Result<()> {
		E::write(self, w)
	}

	fn read_hacked<E: Endian, R: Read>(r: R) -> Result<Self> {
		E::read(r)
	}
}

impl<T: Primitive> HackedPrimitive for T {}

pub trait HackedIo: Io {
	fn write_hacked<E: Endian, W: Write>(&self, w: W) -> Result<()> {
		Io::write(self, w)
	}

	fn read_hacked<E: Endian, R: Read>(r: R) -> Result<Self> {
		Io::read(r)
	}
}

impl<T: Io> HackedIo for T {}
