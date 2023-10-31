use crate::Gui;

impl Gui {
    pub fn resource_info_panel(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::right("right")
            .resizable(true)
            .width_range(100.0..=ui.available_width() / 2.0)
            .show_inside(ui, |ui| {
                if let Some(name) = self.resource_name {
                    if let Some(info) = self.infos.get(&name) {
                        let json_lines: Vec<&str> = info
                            .split_inclusive('\n')
                            // .map(|s| s.to_string())
                            .collect();
                        let text_style = egui::TextStyle::Body;
                        egui::ScrollArea::vertical()
                            .id_source("code_scroll")
                            .show_rows(
                                ui,
                                ui.text_style_height(&text_style),
                                json_lines.len(),
                                |ui, row_range| {
                                    let content: String = row_range
                                        .into_iter()
                                        .map(|i| *json_lines.get(i).unwrap())
                                        .collect();
                                    ui.set_min_width(ui.available_width());
                                    selectable_text(ui, content.as_str());
                                },
                            );
                    }
                }
            });
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> egui::Response {
    ui.add(egui::TextEdit::multiline(&mut text))
}
