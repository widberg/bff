use std::path::Path;

use bff::names::{WORDLIST_ANIMALS, WORDLIST_BIP39};
use clap::ValueEnum;

use crate::error::BffCliResult;
use crate::extract::read_bigfile;

#[derive(ValueEnum, Clone, Copy)]
pub enum Wordlist {
    Animals,
    BIP39,
}

pub fn names(bigfile_path: &Path, wordlist: &Wordlist) -> BffCliResult<()> {
    let bigfile = read_bigfile(bigfile_path)?;

    match wordlist {
        Wordlist::Animals => {
            for name in bigfile.objects.keys() {
                println!("{}", name.get_wordlist_encoded_string(WORDLIST_ANIMALS));
            }
        }
        Wordlist::BIP39 => {
            for name in bigfile.objects.keys() {
                println!("{}", name.get_wordlist_encoded_string(WORDLIST_BIP39));
            }
        }
    };

    Ok(())
}
