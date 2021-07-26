use crate::{Endian, Primitive};
use std::io::{Read, Result, Write};

/// Allows to write primitive types with different representation of bytes
pub trait EndianWriter: Write {
    #[inline]
    fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()> {
        E::write(primitive, self)
    }

    #[inline]
    fn write_ne<T: Primitive>(&mut self, primitive: T) -> Result<()> {
        self.try_write::<crate::NativeEndian, T>(primitive)
    }

    #[inline]
    fn write_le<T: Primitive>(&mut self, primitive: T) -> Result<()> {
        self.try_write::<crate::LittleEndian, T>(primitive)
    }

    #[inline]
    fn write_be<T: Primitive>(&mut self, primitive: T) -> Result<()> {
        self.try_write::<crate::BigEndian, T>(primitive)
    }
}

impl<W: Write + ?Sized> EndianWriter for W {}

/// Allows to read primitive types with different representation of bytes
pub trait EndianReader: Read {
    #[inline]
    fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T> {
        E::read(self)
    }

    #[inline]
    fn read_ne<T: Primitive>(&mut self) -> Result<T> {
        self.try_read::<crate::NativeEndian, T>()
    }

    #[inline]
    fn read_le<T: Primitive>(&mut self) -> Result<T> {
        self.try_read::<crate::LittleEndian, T>()
    }

    #[inline]
    fn read_be<T: Primitive>(&mut self) -> Result<T> {
        self.try_read::<crate::BigEndian, T>()
    }
}

impl<R: Read + ?Sized> EndianReader for R {}
