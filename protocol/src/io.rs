use std::io;
use std::io::{Cursor, Read};

use anyhow::Result;
use byteorder::ReadBytesExt;

/// Trait implemented for types which can be read
/// from a buffer.
pub trait Readable {
    /// Reads this type from the given buffer.
    fn read(buffer: &mut Cursor<&[u8]>) -> Result<Self>
    where
        Self: Sized;
}

/// A variable-length integer as defined by the Minecraft protocol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    fn read(buffer: &mut Cursor<&[u8]>) -> Result<Self>
    where
        Self: Sized,
    {
        Self::read_from(buffer).map_err(Into::into)
    }
}

impl VarInt {
    pub fn read_from(mut reader: impl Read) -> io::Result<Self> {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = reader.read_u8()?;
            let value = i32::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt too long (max length: 5)",
                ));
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(VarInt(result))
    }
}

impl From<VarInt> for i32 {
    fn from(x: VarInt) -> Self {
        x.0
    }
}

impl From<i32> for VarInt {
    fn from(x: i32) -> Self {
        Self(x)
    }
}
