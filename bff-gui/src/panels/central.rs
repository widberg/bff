use std::sync::Arc;

use crate::artifact::Artifact;
// use crate::helpers::image::get_image_widget;
use crate::views::image::image_view;
use crate::views::mesh::mesh_view;
use crate::views::sound::sound_view;
use crate::Gui;

impl Gui {
    pub fn preview_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(resource_name) = self.resource_name {
                let artifact = self.artifacts.get(&resource_name);
                if let Some(a) = artifact {
                    match a {
                        Artifact::Bitmap { is_dds: _, data } => {
                            image_view(ui, &resource_name, data);
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
}
