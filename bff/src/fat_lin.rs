use std::collections::HashMap;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::BffResult;

pub struct FatEntry {
    pub path: PathBuf,
    pub offset: u64,
    pub size: usize,
}

#[derive(Default)]
pub struct Fat {
    pub entries: Vec<FatEntry>,
}

impl Fat {
    pub fn read<R: BufRead>(reader: &mut R) -> BffResult<Fat> {
        let mut fat = Fat::default();

        for line in reader.lines() {
            let line = line?;
            let components = line.rsplitn(3, ' ').collect::<Vec<_>>();
            assert_eq!(components.len(), 3); // TODO: Use a real error
            fat.entries.push(FatEntry {
                path: PathBuf::from(components[2]),
                offset: components[1].parse()?,
                size: components[0].parse()?,
            });
        }

        Ok(fat)
    }
}

#[derive(Default)]
pub struct Lin {
    pub files: HashMap<PathBuf, Vec<u8>>,
}

impl Lin {
    pub fn read<R: Read + Seek>(reader: &mut R, fat: Fat) -> BffResult<Lin> {
        let mut lin = Lin::default();

        for entry in fat.entries {
            let mut file = vec![0; entry.size];
            reader.seek(SeekFrom::Start(entry.offset))?;
            reader.read_exact(&mut file)?;
            lin.files.insert(entry.path, file);
        }

        Ok(lin)
    }
}

pub fn read_fat_lin<F: BufRead + Seek, L: Read + Seek>(
    fat_reader: &mut F,
    lin_reader: &mut L,
) -> BffResult<Lin> {
    let fat = Fat::read(fat_reader)?;
    let lin = Lin::read(lin_reader, fat)?;
    Ok(lin)
}
