macro_rules! versions_enum {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        pub enum Version {
            $($i,)*
        }
    };
}

macro_rules! version_strings_to_versions {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        pub fn version_string_to_version(version_string: &str) -> Option<Version> {
            match version_string {
                $($s => Some(Version::$i),)*
                _ => None,
            }
        }
    };
}

macro_rules! versions_to_version_strings {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        pub fn version_to_version_string(version: Version) -> &'static str {
            match version {
                $(Version::$i => $s,)*
            }
        }
    };
}

macro_rules! versions {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        versions_enum!($(($i,$s)),*);
        version_strings_to_versions!($(($i,$s)),*);
        versions_to_version_strings!($(($i,$s)),*);
    };
}

versions! {
    (V1_06_63_02, "v1.06.63.02 - Asobo Studio - Internal Cross Technology"),
    (V1_220_50_07, "v1.220.50.07 - Asobo Studio - Internal Cross Technology"),
    (V1_231_88_06, "v1.231.88.06 - Asobo Studio - Internal Cross Technology"),
    (V1_235_88_06, "v1.235.88.06 - Asobo Studio - Internal Cross Technology"),
    (V1_258_91_06, "v1.258.91.06 - Asobo Studio - Internal Cross Technology"),
    (V1_278_94_06, "v1.278.94.06 - Asobo Studio - Internal Cross Technology"),
    (V1_286_98_06, "v1.286.98.06 - Asobo Studio - Internal Cross Technology"),
    (V1_290_01_06, "v1.290.01.06 - Asobo Studio - Internal Cross Technology"),
    (V1_290_98_06, "v1.290.98.06 - Asobo Studio - Internal Cross Technology"),
    (V1_290_99_06, "v1.290.99.06 - Asobo Studio - Internal Cross Technology"),
    (V1_291_03_06, "v1.291.03.06 - Asobo Studio - Internal Cross Technology"),
    (V1_291_03_07, "v1.291.03.07 - Asobo Studio - Internal Cross Technology"),
    (V1_325_50_07, "v1.325.50.07 - Asobo Studio - Internal Cross Technology"),
    (V1_377_60_04, "v1.377.60.04 - Asobo Studio - Internal Cross Technology"),
    (V1_379_60_09, "v1.379.60.09 - Asobo Studio - Internal Cross Technology"),
    (V1_381_64_09, "v1.381.64.09 - Asobo Studio - Internal Cross Technology"),
    (V1_381_65_09, "v1.381.65.09 - Asobo Studio - Internal Cross Technology"),
    (V1_381_66_09, "v1.381.66.09 - Asobo Studio - Internal Cross Technology"),
    (V1_381_67_05, "v1.381.67.05 - Asobo Studio - Internal Cross Technology"),
    (V1_381_67_09, "v1.381.67.09 - Asobo Studio - Internal Cross Technology"),
    (V1_381_67_12, "v1.381.67.12 - Asobo Studio - Internal Cross Technology"),
}
