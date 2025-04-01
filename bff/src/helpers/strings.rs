use std::io::Write;
use std::str::from_utf8;

use bff_derive::ReferencedNames;
use binrw::io::{Read, Seek};
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian, Error, NullString, args};
use derive_more::{Constructor, Deref, DerefMut, Display, Error, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    PartialEq,
    Eq,
    Default,
    Deref,
    DerefMut,
    Display,
    Debug,
    From,
    Into,
    Serialize,
    Hash,
    Deserialize,
    ReferencedNames,
)]
#[serde(transparent)]
pub struct FixedStringNull<const S: usize>(pub String);

#[derive(Debug, Display, Error, Constructor)]
#[display(
    "FixedStringNull: expected null terminated string of maximum length {0}, read {0} bytes and did not get a null terminator",
    expected_length
)]
pub struct FixedStringNullUnterminatedError {
    pub expected_length: usize,
}

#[derive(Debug, Display, Error, Constructor)]
#[display(
    "FixedStringNull: expected null terminated string of maximum length {} + 1 null terminator, string was {} bytes long + 1 null terminator",
    maximum_length,
    actual_length
)]
pub struct FixedStringNullTooLongError {
    pub maximum_length: usize,
    pub actual_length: usize,
}

impl<const S: usize> BinRead for FixedStringNull<S> {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let begin = reader.stream_position()?;
        let values = Vec::<u8>::read_options(reader, endian, args! { count: S })?;

        let null_terminator = values.iter().position(|&c| c == b'\0');

        if let Some(null_terminator) = null_terminator {
            match from_utf8(&values[..null_terminator]) {
                Ok(value) => Ok(Self(value.to_owned())),
                Err(e) => Err(Error::Custom {
                    pos: begin + e.valid_up_to() as u64,
                    err: Box::new(e),
                }),
            }
        } else {
            Err(Error::Custom {
                pos: reader.stream_position()?,
                err: Box::new(FixedStringNullUnterminatedError::new(S)),
            })
        }
    }
}

impl<const S: usize> BinWrite for FixedStringNull<S> {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let bytes = self.as_bytes();
        if bytes.len() + 1 > S {
            return Err(Error::Custom {
                pos: writer.stream_position()?,
                err: Box::new(FixedStringNullTooLongError::new(S - 1, bytes.len())),
            });
        }
        writer.write_all(bytes)?;
        writer.write_all(&vec![0; S - bytes.len()])?;
        Ok(())
    }
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deref,
    DerefMut,
    Display,
    From,
    Serialize,
    Deserialize,
    ReferencedNames,
)]
#[serde(transparent)]
pub struct PascalString(pub String);

impl BinRead for PascalString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let count: usize = <u32>::read_options(reader, endian, ())? as usize;

        let ascii_string_position = reader.stream_position()?;

        let value = <Vec<u8>>::read_options(
            reader,
            endian,
            args! {
                count,
            },
        )?;

        match from_utf8(&value) {
            Ok(value) => Ok(Self(value.to_owned())),
            Err(e) => Err(Error::Custom {
                pos: ascii_string_position + e.valid_up_to() as u64,
                err: Box::new(e),
            }),
        }
    }
}

impl BinWrite for PascalString {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        let bytes = self.as_bytes();
        <u32>::write_options(&(bytes.len() as u32), writer, endian, ())?;
        writer.write_all(bytes)?;
        Ok(())
    }
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deref,
    DerefMut,
    Display,
    From,
    Serialize,
    Hash,
    Deserialize,
    ReferencedNames,
)]
#[serde(transparent)]
pub struct PascalStringNull(pub String);

impl BinRead for PascalStringNull {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let count: usize = <u32>::read_options(reader, endian, ())? as usize;

        let begin = reader.stream_position()?;

        let value = <Vec<u8>>::read_options(
            reader,
            endian,
            args! {
                count: count - 1,
            },
        )?;

        // Consume the null terminator
        <u8>::read_options(reader, endian, ())?;

        match from_utf8(&value) {
            Ok(value) => Ok(Self(value.to_owned())),
            Err(e) => Err(Error::Custom {
                pos: begin + e.valid_up_to() as u64,
                err: Box::new(e),
            }),
        }
    }
}

impl BinWrite for PascalStringNull {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        let bytes = self.as_bytes();
        <u32>::write_options(&(bytes.len() as u32 + 1), writer, endian, ())?;
        writer.write_all(bytes)?;
        writer.write_be(&0u8)?;
        Ok(())
    }
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deref,
    DerefMut,
    Display,
    From,
    Serialize,
    Hash,
    Deserialize,
    ReferencedNames,
)]
#[serde(transparent)]
pub struct StringUntilNull(pub String);

impl BinRead for StringUntilNull {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let ascii_string_position = reader.stream_position()?;
        let value = NullString::read(reader)?;
        match from_utf8(&value) {
            Ok(value) => Ok(Self(value.to_owned())),
            Err(e) => Err(Error::Custom {
                pos: ascii_string_position + e.valid_up_to() as u64,
                err: Box::new(e),
            }),
        }
    }
}

impl BinWrite for StringUntilNull {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let value = self.0.as_bytes();
        writer.write_all(value)?;
        writer.write_be(&0u8)?;

        Ok(())
    }
}

impl ReadEndian for StringUntilNull {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl WriteEndian for StringUntilNull {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}
