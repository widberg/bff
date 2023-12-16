use std::io::{Read, Seek, Write};

use crate::bigfile::BigFile;
use crate::names::NameType;
use crate::bigfile::platforms::Platform;
use crate::bigfile::versions::Version;
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

    const NAME_TYPE: NameType;

    type ResourceType;
}
