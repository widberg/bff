use std::ffi::OsStr;

use binrw::Endian;

use crate::BffResult;
use crate::macros::platforms::platforms;

// Add new platforms to the end of this list
// The order of the list is used to generate the platform id used in the BFF resource header
platforms! {
    styles: [BF, D, DB],
    platforms: [
        PC([D("DPC", "NPC"), BF("BFPC", "BFNamePC"), DB("DBC", "NBC")], Little),
        UWP([D("DUA", "NUA")], Little),
        Maci386([D("DMC", "NMC")], Little),
        MacPPC([D("DBM", "NBM")], Big),
        PS2([D("DPS", "NPS"), BF("BFPS2", "BFNamePS2"), DB("DBS", "NBS")], Little),
        PS3([D("DP3", "NP3")], Big),
        PS4([D("DP4", "NP4")], Little),
        PS5([D("DP5", "NP5")], Little),
        PSP([D("DPP", "NPP"), BF("BFPSP", "BFNamePSP")], Little),
        Xbox([D("DXB", "NXB")], Little),
        Xbox360([D("D36", "N36")], Big),
        XboxOne([D("DXO", "NXO")], Little),
        GameCube([D("DGC", "NGC")], Big),
        Wii([D("DRV", "NRV"), BF("BFWii", "BFNameWii"), DB("DBR", "NBR")], Big),
        Switch([D("DNX", "NNX")], Little),
        XboxSeries([D("DXS", "NXS")], Little),
    ]
}

pub fn try_extension_to_endian(extension: &OsStr) -> BffResult<Endian> {
    extension.try_into().map(<Platform as Into<Endian>>::into)
}
