use std::io::{Read, Seek, Write};

use crate::bigfile::BigFile;
use crate::names::NameType;
use crate::platforms::Platform;
use crate::versions::Version;
use crate::BffResult;

pub trait BigFileIo {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile>;

    fn write<W: Write + Seek>(
        bigfile: &BigFile,
        writer: &mut W,
        tag: Option<&str>,
    ) -> BffResult<()>;

    fn name_type(version: Version, platform: Platform) -> NameType;
}
