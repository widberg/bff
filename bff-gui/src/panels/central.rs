use std::collections::HashMap;
use std::sync::Arc;

use bff::names::Name;

use crate::artifact::Artifact;
use crate::helpers::image::get_image;
use crate::views::mesh::mesh_view;
use crate::views::sound::sound_view;

pub fn preview_panel(
    ui: &mut egui::Ui,
    resource_name: &Option<Name>,
    artifacts: &HashMap<Name, Artifact>,
) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(resource_name) = resource_name {
            let artifact = artifacts.get(resource_name);
            if let Some(a) = artifact {
                match a {
                    Artifact::Bitmap { is_dds: _, data } => {
                        ui.add(get_image(resource_name, data));
                    }
                    Artifact::Sound {
                        data,
                        sample_rate,
                        channels,
                    } => {
                        sound_view(
                            ui,
                            "sound".into(),
                            Arc::clone(data),
                            *sample_rate,
                            *channels,
                        );
                    }
                    Artifact::Mesh(model) => {
                        mesh_view(ui, Arc::clone(model));
                    }
                    Artifact::Skin(skin) => {
                        mesh_view(ui, Arc::clone(skin));
                    }
                }
            }
        }
    });
}
