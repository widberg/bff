use bff::crc32::reverse_asobo;
use BffCliError::NoFillerFound;

use crate::error::{BffCliError, BffCliResult};

pub const DEFAULT_CHARACTER_SET: &str = "-.0123456789>_abcdefghijklmnopqrstuvwxyz";

pub fn reverse_crc32(
    string: &str,
    target: &i32,
    starting: &i32,
    min_filler_length: &usize,
    max_filler_length: &usize,
    character_set: &str,
) -> BffCliResult<()> {
    let starting = *starting;
    let target = *target;
    let min_filler_length = *min_filler_length;
    let max_filler_length = *max_filler_length;

    let insert_position = string
        .chars()
        .position(|c| c == '*')
        .unwrap_or(string.len());
    let string = string.replacen('*', "", 1);

    let filled = reverse_asobo(
        &string,
        character_set,
        target,
        starting,
        min_filler_length,
        max_filler_length,
        insert_position,
    );

    match filled {
        Some(filled) => {
            println!(r#"{} "{}""#, target, filled);
            Ok(())
        }
        None => {
            println!("No filler found");
            Err(NoFillerFound {
                min_filler_length,
                max_filler_length,
            })
        }
    }
}
