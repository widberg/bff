use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use bff::names::{Name, NameContext, NameType};
use bff::tsc::{Cps, read_default_cps_names};
use bff::{BufReader, Endian};
use pathdiff::diff_paths;

use crate::error::BffCliResult;
use crate::lz::LzEndian;
use crate::shared::{read_in_names, write_names};

fn collect_cps_names(cps: &Cps, name_context: &mut NameContext) -> HashSet<Name> {
    let mut names = HashSet::new();

    for (path, script) in &cps.tscs {
        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            let name = name_context
                .name_type()
                .parse_name_value(stem)
                .unwrap_or_else(|| name_context.insert(stem));
            names.insert(name);
        }
        for line in script.lines() {
            if let Some(command_name) = line.split_whitespace().next()
                && !command_name.is_empty()
            {
                let name = name_context
                    .name_type()
                    .parse_name_value(command_name)
                    .unwrap_or_else(|| name_context.insert(command_name));
                names.insert(name);
            }
        }
    }

    names
}

pub fn extract_cps(
    cps: &Path,
    directory: &Path,
    in_names: &[PathBuf],
    endian: LzEndian,
) -> BffCliResult<()> {
    let mut name_context = NameContext::new(NameType::BlackSheep32);
    let endian: Endian = endian.into();
    if in_names.is_empty() {
        read_default_cps_names(&mut name_context)?;
    } else {
        read_in_names(in_names, &mut name_context)?;
    }
    let mut cps_reader = BufReader::new(File::open(cps)?);
    let cps = Cps::read(&mut cps_reader, endian, &name_context)?;

    // See comment in FAT/LIN file
    let directory = directory.join("System");

    for (path, data) in cps.tscs {
        let tsc_path = directory.join(path);
        let prefix = tsc_path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let tsc = File::create(tsc_path)?;
        let mut tsc_writer = BufWriter::new(tsc);
        tsc_writer.write_all(data.as_bytes())?;
    }

    Ok(())
}

fn read_files_into_cps_recursively(
    cps: &mut Cps,
    directory: &Path,
    base: &Path,
) -> BffCliResult<()> {
    let paths = std::fs::read_dir(directory)?;
    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            read_files_into_cps_recursively(cps, &path, base)?;
        } else {
            let tsc = std::fs::read_to_string(&path)?;
            let relative_path = diff_paths(&path, base).unwrap();
            cps.tscs.insert(relative_path, tsc);
        }
    }
    Ok(())
}

pub fn create_cps(
    directory: &Path,
    cps_path: &Path,
    out_names: Option<&Path>,
    endian: LzEndian,
    unencrypted: bool,
) -> BffCliResult<()> {
    let mut name_context = NameContext::new(NameType::BlackSheep32);
    let endian: Endian = endian.into();
    let mut cps = Cps::default();
    let directory_cwd = directory.join("System");
    read_files_into_cps_recursively(&mut cps, directory, &directory_cwd)?;
    let names = collect_cps_names(&cps, &mut name_context);

    let mut cps_writer = BufWriter::new(File::create(cps_path)?);

    cps.write(&mut cps_writer, endian, unencrypted, &mut name_context)?;

    if let Some(out_names) = out_names {
        let names: Vec<Name> = names.into_iter().collect();
        write_names(out_names, Some(names.as_slice()), &name_context)?;
    }

    Ok(())
}
