use crate::{Endian, EndiannessDependentIo, Primitive};
use std::io::{Read, Result, Write};

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

pub trait HackedIo: EndiannessDependentIo {
    #[cfg_attr(feature = "inline_io", inline(always))]
    fn write_hacked<E: Endian, W: Write>(&self, w: W) -> Result<()> {
        EndiannessDependentIo::write_with_endianness::<E, W>(self, w)
    }

    #[cfg_attr(feature = "inline_io", inline(always))]
    fn read_hacked<E: Endian, R: Read>(r: R) -> Result<Self> {
        EndiannessDependentIo::read_with_endianness::<E, R>(r)
    }
}

impl<T: EndiannessDependentIo> HackedIo for T {}
