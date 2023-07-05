use ascii::AsciiChar;
use ascii::AsciiString;
use ascii::FromAsciiError;
use binrw::io::Read;
use binrw::io::Seek;
use binrw::BinRead;
use binrw::BinResult;
use binrw::Endian;
use serde::Serialize;
use serde::Serializer;
use std::fmt;

#[derive(Clone, PartialEq, Default)]
pub struct FixedStringNULL<const S: usize>(pub AsciiString);
#[derive(Clone, PartialEq, Default)]
pub struct PascalString(pub String);

impl<const S: usize> BinRead for FixedStringNULL<S> {
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
            // TODO: Don't unwrap
            values.push(AsciiChar::from_ascii(val).unwrap());
        }

        // TODO: Don't unwrap
        reader
            .seek(std::io::SeekFrom::Current(
                i64::try_from(S - values.len() - 1).unwrap(),
            ))
            .unwrap();

        return Ok(Self(values));
    }
}

impl<const S: usize> TryFrom<String> for FixedStringNULL<S> {
    type Error = FromAsciiError<std::string::String>;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(AsciiString::from_ascii(s)?))
    }
}

impl<const S: usize> From<FixedStringNULL<S>> for AsciiString {
    fn from(s: FixedStringNULL<S>) -> Self {
        s.0
    }
}

impl<const S: usize> From<FixedStringNULL<S>> for String {
    fn from(value: FixedStringNULL<S>) -> Self {
        String::from(value.0)
    }
}

impl<const S: usize> core::ops::Deref for FixedStringNULL<S> {
    type Target = AsciiString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const S: usize> core::ops::DerefMut for FixedStringNULL<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const S: usize> fmt::Debug for FixedStringNULL<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const S: usize> fmt::Display for FixedStringNULL<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const S: usize> Serialize for FixedStringNULL<S> {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl BinRead for PascalString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let size = <u32>::read_options(reader, endian, ())?;
        let mut values = String::with_capacity(size as usize);

        for _ in 0..size {
            let val = <u8>::read_options(reader, endian, ())?;
            values.push(val as char);
        }

        // TODO: Don't unwrap
        // reader
        //     .seek(std::io::SeekFrom::Current(
        //         i64::try_from(size as usize - values.len()).unwrap(),
        //     ))
        //     .unwrap();

        return Ok(Self(values));
    }
}

impl fmt::Debug for PascalString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for PascalString {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        self.0.serialize(serializer)
    }
}
