use derive_more::Display;
use scanf::sscanf;

use crate::error::InvalidVersionError;

#[derive(Debug, Display, Copy, Clone)]
pub enum Version {
    #[display(
        fmt = "v{:02}.{:02}.{:02}.{:02} - Asobo Studio - Internal Cross Technology",
        "_0",
        "_1",
        "_2",
        "_3"
    )]
    Asobo(u32, u32, u32, u32),
    #[display(
        fmt = "v{:02}.{:02} - Asobo Studio - Internal Cross Technology",
        "_0",
        "_1"
    )]
    AsoboLegacy(u32, u32),
}

impl TryFrom<&str> for Version {
    type Error = InvalidVersionError;
    fn try_from(value: &str) -> Result<Self, InvalidVersionError> {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        let mut tweak = 0;

        if sscanf!(
            value,
            "v{:02}.{:02}.{:02}.{:02} - Asobo Studio - Internal Cross Technology",
            major,
            minor,
            patch,
            tweak
        )
        .is_ok()
        {
            Ok(Self::Asobo(major, minor, patch, tweak))
        } else if sscanf!(
            value,
            "v{:02}.{:02} - Asobo Studio - Internal Cross Technology",
            major,
            minor,
        )
        .is_ok()
        {
            Ok(Self::AsoboLegacy(major, minor))
        } else {
            Err(InvalidVersionError::new(value.to_string()))
        }
    }
}

pub type VersionTriple = (u32, u32, u32);
