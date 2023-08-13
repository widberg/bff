use ascii::{AsciiChar, AsciiString};
use binrw::io::{Read, Seek};
use binrw::{BinRead, BinResult, Endian, Error, VecArgs};
use derive_more::{Constructor, Deref, DerefMut, Display, Error, From, Into};
use serde::Serialize;

#[derive(
    Clone, PartialEq, Eq, Default, Deref, DerefMut, Display, Debug, From, Into, Serialize, Hash,
)]
#[serde(transparent)]
pub struct FixedStringNull<const S: usize>(pub AsciiString);

#[derive(Debug, Display, Error, Constructor)]
#[display(
    fmt = "FixedStringNull: expected null terminated string of maximum length {}, read {} bytes and did not get a null terminator",
    expected_length,
    expected_length
)]
pub struct FixedStringNullUnterminated {
    pub expected_length: usize,
}

impl<const S: usize> BinRead for FixedStringNull<S> {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut values = AsciiString::with_capacity(S);

        loop {
            let val = <u8>::read_options(reader, endian, ())?;
            if val == 0 {
                break;
            }

            if values.len() == S {
                return Err(Error::Custom {
                    pos: reader.stream_position()? - 1,
                    err: Box::new(FixedStringNullUnterminated::new(S)),
                });
            }

            values.push(match AsciiChar::from_ascii(val) {
                Ok(val) => val,
                Err(e) => {
                    return Err(Error::Custom {
                        pos: reader.stream_position()? - 1,
                        err: Box::new(e),
                    })
                }
            });
        }

        reader.seek(std::io::SeekFrom::Current((S - values.len() - 1) as i64))?;

        Ok(Self(values))
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug, Deref, DerefMut, Display, From, Serialize)]
#[serde(transparent)]
pub struct PascalString(pub AsciiString);

impl BinRead for PascalString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let count: usize = <u32>::read_options(reader, endian, ())? as usize;

        let ascii_string_position = reader.stream_position()?;

        let val = <Vec<u8>>::read_options(
            reader,
            endian,
            VecArgs {
                count,
                inner: <_>::default(),
            },
        )?;

        let values = match AsciiString::from_ascii(val) {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::Custom {
                    pos: ascii_string_position + e.ascii_error().valid_up_to() as u64,
                    err: Box::new(e),
                })
            }
        };

        Ok(Self(values))
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug, Deref, DerefMut, Display, From, Serialize, Hash)]
#[serde(transparent)]
pub struct PascalStringNull(pub AsciiString);

impl BinRead for PascalStringNull {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let count: usize = <u32>::read_options(reader, endian, ())? as usize;

        let ascii_string_position = reader.stream_position()?;

        let val = <Vec<u8>>::read_options(
            reader,
            endian,
            VecArgs {
                count: count - 1,
                inner: <_>::default(),
            },
        )?;

        // Consume the null terminator
        <u8>::read_options(reader, endian, ())?;

        let values = match AsciiString::from_ascii(val) {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::Custom {
                    pos: ascii_string_position + e.ascii_error().valid_up_to() as u64,
                    err: Box::new(e),
                })
            }
        };

        Ok(Self(values))
    }
}
