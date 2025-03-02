use std::collections::HashMap;

use bff::bigfile::BigFile;
use bff::class::bitmap::generic::BitmapGeneric;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;

use crate::artifact::Artifact;
use crate::traits::export::{Export, RecursiveExport};

pub fn create_artifact(bigfile: &BigFile, class: Class) -> Option<Artifact> {
    match class {
        Class::Bitmap(box_bitmap) => {
            let generic = BitmapGeneric::from(*box_bitmap);
            let artifact = generic.export();
            Some(artifact)
        }
        Class::Sound(box_sound) => {
            let artifact = match *box_sound {
                bff::class::sound::Sound::SoundV1_291_03_06PC(sound) => {
                    // let points = sound.body.data.iter().enumerate().map(|(i, s)| eframe::epaint::Pos2{x: ((i as f32 * ui.available_width()) / sound.body.data.len() as f32), y: (s / 200 + 200).into()}).collect();
                    // let shape = eframe::epaint::PathShape::line(points, eframe::epaint::Stroke::new(1.0, eframe::epaint::Color32::WHITE));
                    // ui.painter().add(shape);
                    sound.export()
                }
                bff::class::sound::Sound::SoundV1_381_67_09PC(sound) => sound.export(),
            };
            Some(artifact)
        }
        Class::Mesh(box_mesh) => match *box_mesh {
            bff::class::mesh::Mesh::MeshV1_291_03_06PC(mesh) => Some(mesh.export()),
            _ => None,
        },
        Class::Skin(box_skin) => match *box_skin {
            bff::class::skin::Skin::SkinV1_291_03_06PC(skin) => {
                // let dependency_names = ;
                let dependency_classes: HashMap<Name, Class> = skin
                    .dependencies()
                    .iter()
                    .filter_map(|n| bigfile.objects.get(n))
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
        },
        _ => None,
    }
}
