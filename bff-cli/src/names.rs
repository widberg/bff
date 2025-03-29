use std::path::{Path, PathBuf};

use bff::names::{get_forced_hash_string, WORDLIST_ANIMALS, WORDLIST_BIP39};
use clap::ValueEnum;

use crate::error::BffCliResult;
use crate::extract::{read_bigfile, read_names, write_names};

#[derive(ValueEnum, Clone, Copy)]
pub enum Wordlist {
    Empty,
    Animals,
    BIP39,
}

pub fn names(
    bigfile_path: &Path,
    wordlist: &Option<Wordlist>,
    in_names: &Vec<PathBuf>,
    out_names: &Option<PathBuf>,
) -> BffCliResult<()> {
    read_names(bigfile_path, in_names)?;

    let bigfile = read_bigfile(bigfile_path)?;

    if let Some(wordlist) = wordlist {
        let mut names_db = bff::names::names().lock().unwrap();
        for name in bigfile.objects.keys() {
            if names_db.get(name).is_none() {
                let string = match wordlist {
                    Wordlist::Empty => "".to_string(),
                    Wordlist::Animals => name.get_wordlist_encoded_string(WORDLIST_ANIMALS),
                    Wordlist::BIP39 => name.get_wordlist_encoded_string(WORDLIST_BIP39),
                };
                names_db.insert(&get_forced_hash_string(name, string));
            }
        }
    }

    write_names(out_names)?;

    Ok(())
}
