use crate::Gui;

impl Gui {
    pub fn bottom_panel(&mut self, ctx: &egui::Context, id_source: egui::Id) {
        egui::TopBottomPanel::bottom(id_source).show(ctx, |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                ui.label(
                    self.resource_name
                        .map_or_else(String::new, |f| f.to_string()),
                );
            });
        });
    }
}
