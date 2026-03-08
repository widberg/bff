use std::io::{Cursor, Read, Seek, Write};
use std::path::Path;

use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, BinWriterExt, Endian, NullString, args};

use crate::BffResult;
use crate::crc::racenet32;
use crate::helpers::DynArray;
use crate::lz::{lzo_compress, lzo_decompress};

const SETTINGS_KEY0: u32 = 0xC2B2AE35;
const SETTINGS_KEY1: u32 = 0xCC9E2D51;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqfelSettingsBin {
    pub root_directory: MqfelSettingsDirectory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqfelSettingsDirectory {
    pub name: String,
    pub files: Vec<MqfelSettingsFile>,
    pub directories: Vec<Self>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqfelSettingsFile {
    pub name: String,
    pub data: Vec<u8>,
}

pub fn mqfel_settings_bin_decrypt_buffer(data: &mut [u8]) {
    let mut state = SETTINGS_KEY0;
    for byte in data {
        *byte ^= (state & 0xFF) as u8;
        state = SETTINGS_KEY1.wrapping_mul((u32::from(*byte) << 24) | (state >> 8));
    }
}

pub fn mqfel_settings_bin_encrypt_buffer(data: &mut [u8]) {
    let mut state = SETTINGS_KEY0;
    for byte in data {
        let original = *byte;
        *byte ^= (state & 0xFF) as u8;
        state = SETTINGS_KEY1.wrapping_mul((u32::from(original) << 24) | (state >> 8));
    }
}

impl BinRead for MqfelSettingsFile {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let name = NullString::read_options(reader, endian, ())?;
        let data = NullString::read_options(reader, endian, ())?;
        Ok(Self {
            name: name.try_into().unwrap(),
            data: data.into(),
        })
    }
}

impl BinWrite for MqfelSettingsFile {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        NullString::from(self.name.clone()).write_options(writer, endian, ())?;
        NullString(self.data.clone()).write_options(writer, endian, ())?;
        Ok(())
    }
}

impl BinRead for MqfelSettingsDirectory {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let name = NullString::read_options(reader, endian, ())?;
        let files = DynArray::<MqfelSettingsFile>::read_options(reader, endian, ())?;
        let directories = DynArray::<Self>::read_options(reader, endian, ())?;
        Ok(Self {
            name: name.try_into().unwrap(),
            files: files.inner,
            directories: directories.inner,
        })
    }
}

impl BinWrite for MqfelSettingsDirectory {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        NullString::from(self.name.clone()).write_options(writer, endian, ())?;
        let files: DynArray<MqfelSettingsFile> = self.files.clone().into();
        files.write_options(writer, endian, ())?;
        let directories: DynArray<Self> = self.directories.clone().into();
        directories.write_options(writer, endian, ())?;
        Ok(())
    }
}

fn decrypt_and_decompress<R: Read>(mut reader: R) -> BffResult<Vec<u8>> {
    let mut encrypted_data = Vec::new();
    reader.read_to_end(&mut encrypted_data)?;

    mqfel_settings_bin_decrypt_buffer(&mut encrypted_data);

    let mut encrypted_cursor = Cursor::new(encrypted_data);
    encrypted_cursor.seek(std::io::SeekFrom::End(-4))?;
    let decompressed_size = encrypted_cursor.read_le::<u32>()? as usize;
    let split = encrypted_cursor.stream_position()? as usize - 4;
    encrypted_cursor.seek(std::io::SeekFrom::Start(0))?;
    let compressed_data = Vec::<u8>::read_args(
        &mut encrypted_cursor,
        args! {
            count: split
        },
    )?;

    lzo_decompress(&compressed_data, decompressed_size)
}

fn compress_and_encrypt<W: Write>(decompressed_data: &[u8], writer: &mut W) -> BffResult<()> {
    let decompressed_size = u32::try_from(decompressed_data.len()).unwrap();

    let mut compressed_data = Cursor::new(Vec::new());
    lzo_compress(decompressed_data, &mut compressed_data)?;

    let mut payload_cursor = Cursor::new(compressed_data.into_inner());
    payload_cursor.seek(std::io::SeekFrom::End(0))?;
    payload_cursor.write_le(&decompressed_size)?;
    let mut encrypted_payload = payload_cursor.into_inner();
    mqfel_settings_bin_encrypt_buffer(&mut encrypted_payload);
    writer.write_all(&encrypted_payload)?;

    Ok(())
}

pub fn mqfel_settings_bin_extract<R: Read>(reader: R) -> BffResult<MqfelSettingsBin> {
    let decompressed_data = decrypt_and_decompress(reader)?;
    let mut decompressed_cursor = Cursor::new(decompressed_data.as_slice());
    let root_directory = decompressed_cursor.read_le::<MqfelSettingsDirectory>()?;
    decompressed_cursor.read_le::<u32>()?;
    let consumed = decompressed_cursor.stream_position()? as usize;
    assert_eq!(
        consumed,
        decompressed_data.len(),
        "MQFEL settings.bin contains unparsed trailing data after checksum"
    );
    Ok(MqfelSettingsBin { root_directory })
}

pub fn mqfel_settings_bin_create<W: Write>(
    settings_bin: &MqfelSettingsBin,
    writer: &mut W,
) -> BffResult<()> {
    let mut inner_data = Cursor::new(Vec::new());
    inner_data.write_le(&settings_bin.root_directory)?;
    let checksum = racenet32(inner_data.get_ref()) as u32;
    inner_data.write_le(&checksum)?;
    let decompressed_data = inner_data.into_inner();
    compress_and_encrypt(&decompressed_data, writer)
}

fn write_simple_directory_to_fs(
    directory: &MqfelSettingsDirectory,
    parent: &Path,
) -> BffResult<()> {
    let directory_path = parent.join(&directory.name);
    std::fs::create_dir_all(&directory_path)?;

    for file in &directory.files {
        let output_file_name = file
            .name
            .rsplit(['\\', '/'])
            .find(|segment| !segment.is_empty())
            .unwrap();
        let file_path = directory_path.join(output_file_name);
        std::fs::write(file_path, &file.data)?;
    }

    for child_directory in &directory.directories {
        write_simple_directory_to_fs(child_directory, &directory_path)?;
    }

    Ok(())
}

fn read_simple_directory_from_fs(
    path: &Path,
    explicit_path_under_root: &[String],
) -> BffResult<MqfelSettingsDirectory> {
    let directory_name = path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut files: Vec<MqfelSettingsFile> = Vec::new();
    let mut directories: Vec<MqfelSettingsDirectory> = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let mut child_explicit_path = explicit_path_under_root.to_vec();
            let child_name = entry.file_name().into_string().unwrap();
            child_explicit_path.push(child_name);
            directories.push(read_simple_directory_from_fs(
                &entry_path,
                &child_explicit_path,
            )?);
        } else if entry_path.is_file() {
            let file_name = entry.file_name().into_string().unwrap();

            let data = std::fs::read(entry_path)?;
            files.push(MqfelSettingsFile {
                name: if explicit_path_under_root.is_empty() {
                    file_name
                } else {
                    format!(
                        "{}\\{}",
                        explicit_path_under_root
                            .iter()
                            .map(|part| part.to_uppercase())
                            .collect::<Vec<_>>()
                            .join("\\"),
                        file_name
                    )
                },
                data,
            });
        }
    }

    files.sort_by_cached_key(|file| {
        file.name
            .rsplit_once('.')
            .map_or_else(|| file.name.to_uppercase(), |(stem, _)| stem.to_uppercase())
    });
    directories.sort_by_cached_key(|directory| directory.name.to_uppercase());

    Ok(MqfelSettingsDirectory {
        name: directory_name,
        files,
        directories,
    })
}

pub fn mqfel_settings_bin_extract_to_directory<R: Read>(
    reader: R,
    output_directory: &Path,
) -> BffResult<()> {
    let settings = mqfel_settings_bin_extract(reader)?;
    std::fs::create_dir_all(output_directory)?;
    write_simple_directory_to_fs(&settings.root_directory, output_directory)?;
    Ok(())
}

pub fn mqfel_settings_bin_create_from_directory<W: Write>(
    input_directory: &Path,
    writer: &mut W,
) -> BffResult<()> {
    let mut root_directory = None;

    for entry in std::fs::read_dir(input_directory)? {
        let entry = entry?;
        let entry_path = entry.path();
        if !entry_path.is_dir() || root_directory.is_some() {
            return Err(std::io::Error::other(
                "MQFEL settings input must contain exactly one root subdirectory and no files",
            )
            .into());
        }
        root_directory = Some(entry_path);
    }

    let root_directory = root_directory.ok_or_else(|| {
        std::io::Error::other(
            "MQFEL settings input must contain exactly one root subdirectory and no files",
        )
    })?;

    let settings = MqfelSettingsBin {
        root_directory: read_simple_directory_from_fs(&root_directory, &[])?,
    };
    mqfel_settings_bin_create(&settings, writer)
}
