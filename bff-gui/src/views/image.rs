use std::sync::Arc;

use bff::names::Name;

use crate::artifact::BitmapFormat;

pub fn image_view(
    ui: &mut egui::Ui,
    resource_name: &Name,
    format: &BitmapFormat,
    data: &Arc<Vec<u8>>,
) {
    let ext = match format {
        BitmapFormat::Dds => "dds",
        BitmapFormat::Raw => "bmp",
    };
    ui.add_sized(
        ui.available_size(),
        egui::Image::from_bytes(format!("bytes://{}.{}", resource_name, ext), data.to_vec())
            .shrink_to_fit(),
    );
}
