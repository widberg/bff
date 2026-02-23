use std::collections::HashMap;

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::versions::Version;
use bff::class::Class;
use bff::class::{ClassNameStyle, ClassType};
use bff::class::bitmap::generic::BitmapGeneric;
use bff::class::sound::generic::SoundGeneric;
use bff::names::{Name, NameType};
use bff::traits::TryIntoVersionPlatform;

use crate::artifact::Artifact;
use crate::traits::export::{Export, RecursiveExport};

pub fn class_supports_preview(class_name: Name, version: &Version, platform: Platform) -> bool {
    let Ok((class_type, _, _)) =
        <(ClassType, ClassNameStyle, NameType)>::try_from(class_name)
    else {
        return false;
    };
    match class_type {
        ClassType::Bitmap => matches!(
            (version, platform),
            (&Version::Asobo(1, 6, 63, 2), Platform::PC)
                | (&Version::Asobo(1, 381, 67, 9), Platform::PC)
        ) || matches!(version, &Version::Asobo(1, minor, _, _) if (231..=291).contains(&minor)),
        ClassType::Sound => matches!(
            (version, platform),
            (&Version::Asobo(1, 381, 67, 9), Platform::PC)
        ) || matches!(version, &Version::Asobo(1, minor, _, _) if (6..=291).contains(&minor)),
        ClassType::Mesh => matches!(
            (version, platform),
            (&Version::Asobo(1, 291, 3, 6), Platform::PC)
        ),
        ClassType::Skin => matches!(
            (version, platform),
            (&Version::Asobo(1, 6, 63, 2), Platform::PC)
                | (&Version::Asobo(1, 291, 3, 6), Platform::PC)
        ),
        _ => false,
    }
}

pub fn create_artifact(bigfile: &BigFile, class: Class) -> Option<Artifact> {
    match class {
        Class::Bitmap(bitmap) => {
            let generic = BitmapGeneric::from(bitmap);
            let artifact = generic.export();
            Some(artifact)
        }
        Class::Sound(sound) => {
            let generic = SoundGeneric::from(sound);
            let artifact = generic.export();
            Some(artifact)
        }
        Class::Mesh(bff::class::mesh::Mesh::MeshV1_291_03_06PC(mesh)) => Some(mesh.export()),
        Class::Skin(bff::class::skin::Skin::SkinV1_291_03_06PC(skin)) => {
            let dependency_classes: HashMap<Name, Class> = skin
                .dependencies()
                .iter()
                .filter_map(|n| bigfile.resources.get(n))
                .map(|r| {
                    (
                        r.name,
                        TryIntoVersionPlatform::<Class>::try_into_version_platform(
                            r,
                            bigfile.manifest.version.clone(),
                            bigfile.manifest.platform,
                        )
                        .unwrap(),
                    )
                })
                .collect();
            Some(skin.export(&dependency_classes))
        }
        _ => None,
    }
}
