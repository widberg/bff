use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use bff::names::{Name, NameContext, NameType};
use bff::tsc::{Cps, read_default_cps_names};
use bff::{BufReader, Endian};
use pathdiff::diff_paths;

use crate::error::BffCliResult;
use crate::extract::{read_in_names, write_names};
use crate::lz::LzEndian;

fn collect_cps_names(cps: &Cps, name_context: &NameContext) -> HashSet<Name> {
    let mut names = HashSet::new();

    for (path, script) in &cps.tscs {
        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            names.insert(name_context.parse_i32_or_hash_name(stem));
        }
        for line in script.lines() {
            if let Some(command_name) = line.split_whitespace().next() {
                if !command_name.is_empty() {
                    names.insert(name_context.parse_i32_or_hash_name(command_name));
                }
            }
        }
    }

    names
}

pub fn extract_cps(
    cps: &Path,
    directory: &Path,
    in_names: &Vec<PathBuf>,
    endian: &LzEndian,
) -> BffCliResult<()> {
    let name_context = NameContext::default();
    let endian: Endian = (*endian).into();
    name_context.set_name_type(NameType::BlackSheep32);
    if in_names.is_empty() {
        read_default_cps_names(&name_context)?;
    } else {
        read_in_names(in_names, &name_context)?;
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
    out_names: &Option<PathBuf>,
    endian: &LzEndian,
    unencrypted: &bool,
) -> BffCliResult<()> {
    let name_context = NameContext::default();
    let endian: Endian = (*endian).into();
    name_context.set_name_type(NameType::BlackSheep32);
    let mut cps = Cps::default();
    let directory_cwd = directory.join("System");
    read_files_into_cps_recursively(&mut cps, directory, &directory_cwd)?;
    let names = collect_cps_names(&cps, &name_context);

    let mut cps_writer = BufWriter::new(File::create(cps_path)?);

    cps.write(&mut cps_writer, endian, *unencrypted, &name_context)?;

    if let Some(out_names) = out_names {
        let names: Vec<&Name> = names.iter().collect();
        write_names(out_names, &Some(names), &name_context)?;
    }

    Ok(())
}
