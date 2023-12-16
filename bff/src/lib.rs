pub mod audio;
pub mod bigfile;
pub mod class;
pub mod crc;
pub mod error;
pub mod fat_lin;
pub mod helpers;
pub mod lz;
pub mod macros;
pub mod names;
pub mod traits;
pub mod tsc;

pub type BffError = crate::error::Error;
pub type BffResult<T> = std::result::Result<T, BffError>;
pub type Endian = binrw::Endian;
pub type BufReader<T> = binrw::io::BufReader<T>;
