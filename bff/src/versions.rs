use derive_more::Display;
use scanf::sscanf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Display, Clone)]
pub enum Version {
    #[display(
        fmt = "v{}.{:02}.{:02}.{:02} - Asobo Studio - Internal Cross Technology",
        "_0",
        "_1",
        "_2",
        "_3"
    )]
    Asobo(u32, u32, u32, u32),
    #[display(
        fmt = "v{}.{:02} - Asobo Studio - Internal Cross Technology",
        "_0",
        "_1"
    )]
    AsoboLegacy(u32, u32),
    Other(String),
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        let mut tweak = 0;

        if sscanf!(
            value,
            "v{}.{}.{}.{} - Asobo Studio - Internal Cross Technology",
            major,
            minor,
            patch,
            tweak
        )
        .is_ok()
        {
            Self::Asobo(major, minor, patch, tweak)
        } else if sscanf!(
            value,
            "v{}.{} - Asobo Studio - Internal Cross Technology",
            major,
            minor,
        )
        .is_ok()
        {
            Self::AsoboLegacy(major, minor)
        } else {
            Self::Other(value.to_string())
        }
    }
}

impl Serialize for Version {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        Ok(string.as_str().into())
    }
}

pub type VersionTriple = (u32, u32, u32);
