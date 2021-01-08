use std::io::{Read, Result, Write};

use crate::{BigEndian, Endian, LittleEndian, NativeEndian, Primitive};

/// Allows to write primitive types with differents representation of bytes
pub trait EndianWriter: Write {
    #[inline]
    fn try_write<E: Endian, T: Primitive>(&mut self, primitive: T) -> Result<()> {
        E::write(primitive, self)
    }

    #[inline]
    fn write_ne<T: Primitive>(&mut self, primitive: T) -> Result<()> {
        self.try_write::<NativeEndian, T>(primitive)
    }

    #[inline]
    fn write_le<T: Primitive>(&mut self, primitive: T) -> Result<()> {
        self.try_write::<LittleEndian, T>(primitive)
    }

    #[inline]
    fn write_be<T: Primitive>(&mut self, primitive: T) -> Result<()> {
        self.try_write::<BigEndian, T>(primitive)
    }
}

impl<W: Write + ?Sized> EndianWriter for W {}

/// Allows to read primitive types with differents representation of bytes
pub trait EndianReader: Read {
    #[inline]
    fn try_read<E: Endian, T: Primitive>(&mut self) -> Result<T> {
        E::read(self)
    }

    #[inline]
    fn read_ne<T: Primitive>(&mut self) -> Result<T> {
        self.try_read::<NativeEndian, T>()
    }

    #[inline]
    fn read_le<T: Primitive>(&mut self) -> Result<T> {
        self.try_read::<LittleEndian, T>()
    }

    #[inline]
    fn read_be<T: Primitive>(&mut self) -> Result<T> {
        self.try_read::<BigEndian, T>()
    }
}

impl<R: Read + ?Sized> EndianReader for R {}
