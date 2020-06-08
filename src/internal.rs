use std::io::{Read, Result, Write};

use crate::{CanIo, Endian, Primitive};

pub trait HackedPrimitive: Primitive {
	fn write_hacked<E: Endian, W: Write>(self, w: W) -> Result<()> {
		E::write(self, w)
	}

	fn read_hacked<E: Endian, R: Read>(r: R) -> Result<Self> {
		E::read(r)
	}
}

pub trait OnlyCanIo {}

pub enum Io {}

impl OnlyCanIo for Io {}

impl<T: Primitive> HackedPrimitive for T {}

pub trait HackedCanIo: CanIo {
	fn write_hacked<E: OnlyCanIo, W: Write>(self, w: W) -> Result<()> {
		CanIo::write(&self, w)
	}

	fn read_hacked<E: OnlyCanIo, R: Read>(r: R) -> Result<Self> {
		CanIo::read(r)
	}
}

impl<T: CanIo> HackedCanIo for T {}
