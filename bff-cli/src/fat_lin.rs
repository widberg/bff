use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use bff::BufReader;
use bff::fat_lin::{Lin, read_fat_lin, write_fat_lin};
use pathdiff::diff_paths;

use crate::error::BffCliResult;

pub fn extract_fat_lin(fat: &Path, lin: &Path, directory: &Path) -> BffCliResult<()> {
    let mut fat = BufReader::new(File::open(fat)?);
    let mut lin = BufReader::new(File::open(lin)?);

    let lin = read_fat_lin(&mut fat, &mut lin)?;

    // Paths start with at most one `..` component in known FAT files
    // We could be a lot safer about this, but this is good enough for now
    // Also for some reason Boot.tsc is repeated in the FAT and LIN files
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

fn read_files_into_lin_recursively(
    lin: &mut Lin,
    directory: &Path,
    base: &Path,
) -> BffCliResult<()> {
    let paths = std::fs::read_dir(directory)?;
    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            read_files_into_lin_recursively(lin, &path, base)?;
        } else {
            let mut f = BufReader::new(File::open(&path)?);
            let mut contents = Vec::new();
            f.read_to_end(&mut contents)?;
            let relative_path = diff_paths(&path, base).unwrap();
            lin.files.insert(relative_path, contents);
        }
    }
    Ok(())
}

pub fn create_fat_lin(directory: &Path, fat_path: &Path, lin_path: &Path) -> BffCliResult<()> {
    let mut lin = Lin::default();
    let directory_cwd = directory.join("__cwd__");
    read_files_into_lin_recursively(&mut lin, directory, &directory_cwd)?;

    let mut fat_writer = BufWriter::new(File::create(fat_path)?);
    let mut lin_writer = BufWriter::new(File::create(lin_path)?);

    write_fat_lin(&lin, &mut fat_writer, &mut lin_writer)?;

    Ok(())
}
