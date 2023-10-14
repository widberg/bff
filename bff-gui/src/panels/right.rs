use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;
use egui;

pub fn resource_info(ui: &mut egui::Ui, bigfile: &Option<BigFile>, resource_name: &Option<Name>) {
    egui::SidePanel::right("right")
        .resizable(true)
        .width_range(100.0..=ui.available_width() / 2.0)
        .show_inside(ui, |ui| {
            if let Some(resource_name) = resource_name {
                let json = match &bigfile
                    .as_ref()
                    .unwrap()
                    .objects
                    .get(resource_name)
                    .unwrap()
                    .try_into_version_platform(
                        bigfile.as_ref().unwrap().manifest.version.clone(),
                        bigfile.as_ref().unwrap().manifest.platform,
                    ) {
                    Ok(v) => serde_json::to_string_pretty::<Class>(v).unwrap(),
                    Err(e) => e.to_string(),
                };
                let json_lines: Vec<&str> = json
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
        });
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> egui::Response {
    ui.add(egui::TextEdit::multiline(&mut text))
}
