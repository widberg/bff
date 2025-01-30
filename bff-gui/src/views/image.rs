use std::sync::Arc;

use bff::names::Name;

pub fn image_view(ui: &mut egui::Ui, resource_name: &Name, data: &Arc<Vec<u8>>) {
    ui.add(
        egui::Image::from_bytes(format!("bytes://{}.dds", resource_name), data.to_vec())
            .shrink_to_fit()
            .sense(egui::Sense::drag()),
    );
}
