use std::ffi::OsStr;

use binrw::Endian;

use crate::macros::platforms::platforms;
use crate::BffResult;

platforms! {
    styles: [BF, D, DB],
    platforms: [
        PC([D("DPC", "NPC"), BF("BFPC", "BFNamePC"), DB("DBC", "NBC")], Little),
        UWP([D("DUA", "NUA")], Little),
        Maci386([D("DMC", "NMC")], Little),
        MacPPC([D("DBM", "NBM")], Big),
        PS2([D("DPS", "NPS")], Little),
        PS3([D("DP3", "NP3")], Big),
        PSP([D("DPP", "NPP"), BF("BFPSP", "BFNamePSP")], Little),
        Xbox([D("DXB", "NXB")], Little),
        Xbox360([D("D36", "N36")], Big),
        GameCube([D("DGC", "NGC")], Big),
        Wii([D("DRV", "NRV"), BF("BFWii", "BFNameWii"), DB("DBR", "NBR")], Big),
        Switch([D("DNX", "NNX")], Little),
    ]
}

pub fn try_extension_to_endian(extension: &OsStr) -> BffResult<Endian> {
    extension.try_into().map(<Platform as Into<Endian>>::into)
}
