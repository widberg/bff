use std::path::PathBuf;

use bff::names::{WORDLIST_ANIMALS, WORDLIST_BIP39, get_forced_hash_string};
use clap::ValueEnum;

use crate::error::BffCliResult;
use crate::extract::{read_bigfile, read_bigfile_names, read_in_names, write_names};

#[derive(ValueEnum, Clone, Copy)]
pub enum Wordlist {
    Empty,
    Animals,
    BIP39,
}

pub fn names(
    bigfile_path: &Option<PathBuf>,
    wordlist: &Option<Wordlist>,
    in_names: &Vec<PathBuf>,
    out_names: &Option<PathBuf>,
) -> BffCliResult<()> {
    if let Some(bigfile_path) = bigfile_path {
        read_bigfile_names(bigfile_path)?;
    }
    read_in_names(in_names)?;

    if let Some(bigfile_path) = bigfile_path {
        read_bigfile_names(bigfile_path)?;

        let bigfile = read_bigfile(bigfile_path, &None, &None)?;

        if let Some(wordlist) = wordlist {
            for resource in bigfile.resources.values() {
                let name = &resource.name;
                let class = resource.class_name.to_string();
                let mut names_db = bff::names::names().lock().unwrap();
                if names_db.get(name).is_none() {
                    let string = match wordlist {
                        Wordlist::Empty => "".to_owned(),
                        Wordlist::Animals => name.get_wordlist_encoded_string(WORDLIST_ANIMALS),
                        Wordlist::BIP39 => name.get_wordlist_encoded_string(WORDLIST_BIP39),
                    };
                    names_db.insert(&get_forced_hash_string(
                        name,
                        format!("{}.{}", string, class),
                    ));
                }
            }
        }

        if let Some(out_names) = out_names {
            write_names(out_names, &Some(bigfile.resources.keys().collect()))?;
        }
    } else if let Some(out_names) = out_names {
        write_names(out_names, &None)?;
    }

    Ok(())
}
