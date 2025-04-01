use std::io::{Read, Seek, Write};

use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::platforms::Platform;
use crate::bigfile::versions::Version;
use crate::names::NameType;

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
