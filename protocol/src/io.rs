use std::io;
use std::io::{Cursor, Read};

use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};

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

macro_rules! integer_impl {
    ($($int:ty, $read_fn:tt),* $(,)?) => {
        $(
            impl Readable for $int {
                fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
                    buffer.$read_fn::<BigEndian>().map_err(anyhow::Error::from)
                }
            }
        )*
    }
}

integer_impl! {
    u16, read_u16,
    u32, read_u32,
    u64, read_u64,

    i16, read_i16,
    i32, read_i32,
    i64, read_i64,

    f32, read_f32,
    f64, read_f64,
}

impl Readable for String {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Length is encoded as VarInt.
        // Following `length` bytes are the UTF8-encoded
        // string.

        let length = VarInt::read(buffer)
            .context("failed to read string length")?
            .0 as usize;

        // TODO: support custom length limits
        // Current max length is max value of a signed 16-bit int.
        let max_length = std::i16::MAX as usize;
        if length > max_length {
            log::error!(
                "string length {} exceeds maximum allowed length of {}",
                length,
                max_length
            );
        }

        // Read string into buffer.
        let mut temp = vec![0u8; length];
        buffer.read_exact(&mut temp)?;
        let s = std::str::from_utf8(&temp).context("string contained invalid UTF8")?;

        Ok(s.to_owned())
    }
}
