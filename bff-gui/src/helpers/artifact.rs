use std::collections::HashMap;
use std::ffi::OsString;
use std::sync::Arc;

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::versions::Version;
use bff::class::bitmap::generic::BitmapGeneric;
use bff::class::{Class, ClassNameStyle, ClassType};
use bff::names::{Name, NameType};
use bff::traits::{Artifact as BffArtifact, Export as BffExport, TryIntoVersionPlatform};

use crate::artifact::{Artifact, BitmapFormat};
use crate::traits::export::{Export, RecursiveExport};

pub fn class_supports_preview(class_name: Name, version: &Version, platform: Platform) -> bool {
    let Ok((class_type, _, _)) = <(ClassType, ClassNameStyle, NameType)>::try_from(class_name)
    else {
        return false;
    };
    match class_type {
        ClassType::Bitmap => matches!(
            (version, platform),
            (&Version::Asobo(1, 6, 63, 2), Platform::PC)
                | (&Version::Asobo(1, 381, 67, 9), Platform::PC)
                | (&Version::Asobo(1, 231..=291, _, _), _)
        ),
        ClassType::Sound => matches!(
            (version, platform),
            (&Version::Asobo(1, 381, 67, 9), Platform::PC) | (&Version::Asobo(1, 6..=291, _, _), _)
        ),
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
            let data_name = OsString::from("data");
            if let Ok(mut exported_artifacts) = BffExport::export(&bitmap)
                && let Some(exported_artifact) = exported_artifacts.remove(&data_name)
            {
                match exported_artifact {
                    BffArtifact::Dds(bytes) => {
                        return Some(Artifact::Bitmap {
                            format: BitmapFormat::Dds,
                            data: Arc::new(bytes),
                        });
                    }
                    BffArtifact::Binary(bytes) => {
                        return Some(Artifact::Bitmap {
                            format: BitmapFormat::Raw,
                            data: Arc::new(bytes),
                        });
                    }
                    BffArtifact::Text(_) => {}
                    BffArtifact::Wav(_) => {}
                }
            }

            let generic = BitmapGeneric::from(bitmap);
            let artifact = generic.export();
            Some(artifact)
        }
        Class::Sound(sound) => {
            let data_name = OsString::from("data");
            if let Ok(mut exported_artifacts) = BffExport::export(&sound)
                && let Some(exported_artifact) = exported_artifacts.remove(&data_name)
            {
                match exported_artifact {
                    BffArtifact::Dds(_) => {}
                    BffArtifact::Binary(_) => {}
                    BffArtifact::Text(_) => {}
                    BffArtifact::Wav(bytes) => {
                        return Some(Artifact::Sound {
                            data: Arc::from(bytes),
                        });
                    }
                }
            }

            None
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
