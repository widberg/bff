use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use bff::fat_lin::read_fat_lin;
use bff::BufReader;

use crate::error::BffCliResult;

pub fn extract_fat_lin(fat: &Path, lin: &Path, directory: &Path) -> BffCliResult<()> {
    let mut fat = BufReader::new(File::open(fat)?);
    let mut lin = BufReader::new(File::open(lin)?);

    let lin = read_fat_lin(&mut fat, &mut lin)?;

    // Paths start with at most one `..` component in known FAT files
    // We could be a lot safer about this, but this is good enough for now
    let directory = directory.join("__cwd__");

    for (path, contents) in lin.files {
        let path = directory.join(path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let mut writer = BufWriter::new(File::create(&path)?);
        writer.write_all(&contents)?;
    }

    Ok(())
}

pub fn create_fat_lin(_directory: &Path, _fat: &Path, _lin: &Path) -> BffCliResult<()> {
    todo!()
}
