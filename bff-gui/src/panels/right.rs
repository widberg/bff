use std::f32;

use crate::Gui;

impl Gui {
    pub fn resource_info_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right")
            .resizable(true)
            .show(ctx, |ui: &mut egui::Ui| {
                if let Some(name) = self.resource_name {
                    if let Some(info) = self.infos.get(&name) {
                        egui::ScrollArea::both()
                            .auto_shrink([false; 2])
                            .id_salt("code_scroll")
                            .show(ui, |ui| {
                                selectable_text(ui, info);
                            });
                    }
                }
            });
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> egui::Response {
    let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
        let mut layout_job = egui::text::LayoutJob::simple(
            string.to_owned(),
            egui::FontId::monospace(10.0),
            egui::Color32::WHITE,
            f32::INFINITY,
        );
        layout_job.wrap.max_width = f32::INFINITY;
        ui.fonts(|f| f.layout_job(layout_job))
    };
    ui.add(
        egui::TextEdit::multiline(&mut text)
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .desired_rows(60)
            .layouter(&mut layouter),
    )
}
