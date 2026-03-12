use crate::Gui;
use crate::artifact::Artifact;
use crate::views::image::image_view;
use crate::views::mesh::mesh_view;
use crate::views::sound::sound_view;

impl Gui {
    pub fn preview_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Some(resource_name) = self.resource_name {
                let artifact = self.artifacts.get(&resource_name);
                if let Some(a) = artifact {
                    match a {
                        Artifact::Bitmap { format, data } => {
                            image_view(ui, &resource_name, format, data);
                        }
                        Artifact::Sound { data } => {
                            sound_view(ui, egui::Id::new(("sound", resource_name)), data.clone());
                        }
                        Artifact::Mesh(model) => {
                            mesh_view(ui, model.clone());
                        }
                        Artifact::Skin(skin) => {
                            mesh_view(ui, skin.clone());
                        }
                    }
                }
            }
        });
    }
}
