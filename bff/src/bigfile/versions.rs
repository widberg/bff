use binrw::{BinRead, BinWrite};
use derive_more::{Display, From};
use scanf::sscanf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::helpers::PascalString;
use crate::names::names;

#[derive(Debug, Display, Clone, Eq, PartialEq, BinRead, BinWrite)]
pub enum Version {
    #[brw(magic = 0u8)]
    #[display(
        "v{}.{:02}.{:02}.{:02} - Asobo Studio - Internal Cross Technology",
        _0,
        _1,
        _2,
        _3
    )]
    Asobo(u16, u16, u16, u16),
    #[brw(magic = 1u8)]
    #[display("v{}.{:02} - Asobo Studio - Internal Cross Technology", _0, _1)]
    AsoboLegacy(u16, u16),
    #[brw(magic = 2u8)]
    #[display(
        "TotemTech Data v{}.{} (c) 1999-2002 Kalisto Entertainment - All right reserved",
        _0,
        _1
    )]
    Kalisto(u16, u16),
    #[brw(magic = 3u8)]
    // The space is intentional :(
    // This format is used in Shaun White Snowboarding: World Stage by Ubisoft as well
    #[display("Bigfile Data v{}.{} ", _0, _1)]
    BlackSheep(u16, u16),
    #[brw(magic = 4u8)]
    // Used in The Mighty Quest for Epic Loot by Ubisoft
    #[display(
        "Opal {}.{} BigFile | Data Version v{}.{} | CVT {} | CVANIM {} | CVMESH {} | CVSHADER {} |",
        opal_version.0,
        opal_version.1,
        data_version.0,
        data_version.1,
        cvt,
        cvanim,
        cvmesh,
        cvshader
    )]
    Ubisoft {
        opal_version: (u16, u16),
        data_version: (u16, u16),
        cvt: u8,
        cvanim: u8,
        cvmesh: u8,
        cvshader: u8,
    },
    #[brw(magic = 5u8)]
    Other(
        #[br(map = |x: PascalString<u8>| x.to_string())]
        #[bw(map = |x| Into::<PascalString<u8>>::into(x.clone()))]
        String,
    ),
}

impl Version {
    pub fn size_on_disk(&self) -> u16 {
        1 + match self {
            Self::Asobo(_, _, _, _) => 2 * 4,
            Self::AsoboLegacy(_, _) => 2 * 2,
            Self::Kalisto(_, _) => 2 * 2,
            Self::BlackSheep(_, _) => 2 * 2,
            Self::Ubisoft { .. } => 2 * 4 + 4,
            Self::Other(x) => 1 + x.len() as u16,
        }
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        #![allow(clippy::just_underscores_and_digits)]
        let (mut _0, mut _1, mut _2, mut _3, mut _4, mut _5, mut _6, mut _7): (
            u16,
            u16,
            u16,
            u16,
            u8,
            u8,
            u8,
            u8,
        ) = Default::default();

        if sscanf!(
            value,
            "v{}.{}.{}.{} - Asobo Studio - Internal Cross Technology",
            _0, _1, _2, _3,
        )
        .is_ok()
        {
            #[allow(clippy::used_underscore_binding)]
            Self::Asobo(_0, _1, _2, _3)
        } else if sscanf!(
            value,
            "v{}.{} - Asobo Studio - Internal Cross Technology",
            _0, _1
        )
        .is_ok()
        {
            #[allow(clippy::used_underscore_binding)]
            Self::AsoboLegacy(_0, _1)
        } else if sscanf!(
            value,
            "TotemTech Data v{}.{} (c) 1999-2002 Kalisto Entertainment - All right reserved",
            _0, _1
        )
        .is_ok()
        {
            #[allow(clippy::used_underscore_binding)]
            Self::Kalisto(_0, _1)
        } else if sscanf!(value, "Bigfile Data v{}.{} ", _0, _1).is_ok() {
            #[allow(clippy::used_underscore_binding)]
            Self::BlackSheep(_0, _1)
        } else if sscanf!(value, "Opal {}.{} BigFile | Data Version v{}.{} | CVT {} | CVANIM {} | CVMESH {} | CVSHADER {} |",
            _0, _1, _2, _3, _4, _5, _6, _7).is_ok() {
            #[allow(clippy::used_underscore_binding)]
            Self::Ubisoft {
                opal_version: (_0, _1),
                data_version: (_2, _3),
                cvt: _4,
                cvanim: _5,
                cvmesh: _6,
                cvshader: _7,
            }
        } else {
            Self::Other(value.to_owned())
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
        let version: Self = string.as_str().into();
        names().lock().unwrap().name_type = (&version).try_into().unwrap(); // FIXME: name_type should not exist
        Ok(version)
    }
}

#[derive(Debug, Clone, Copy, BinRead, BinWrite, Serialize, Deserialize, From)]
#[serde(untagged)]
pub enum VersionXple {
    Oneple(VersionOneple),
    Triple(VersionTriple),
}

pub type VersionOneple = u32;
pub type VersionTriple = (u32, u32, u32);
