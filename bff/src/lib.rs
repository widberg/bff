pub mod bigfile;
pub mod block;
pub mod class;
pub mod crc32;
pub mod crc64;
pub mod dynarray;
pub mod error;
pub mod header;
pub mod lz;
pub mod math;
pub mod name;
pub mod object;
pub mod platforms;
pub mod pool;
pub mod strings;
pub mod traits;
pub mod versions;
pub mod option;

pub type BffResult<T> = std::result::Result<T, crate::error::Error>;
pub type Endian = binrw::Endian;
pub type BufReader<T> = binrw::io::BufReader<T>;
