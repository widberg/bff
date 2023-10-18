pub mod audio;
pub mod bigfile;
pub mod class;
pub mod crc;
pub mod csc;
pub mod error;
pub mod helpers;
pub mod lz;
pub mod macros;
pub mod names;
pub mod platforms;
pub mod psc;
pub mod traits;
pub mod versions;

pub type BffError = crate::error::Error;
pub type BffResult<T> = std::result::Result<T, BffError>;
pub type Endian = binrw::Endian;
pub type BufReader<T> = binrw::io::BufReader<T>;
