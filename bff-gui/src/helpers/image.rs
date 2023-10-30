use std::sync::Arc;

use bff::names::Name;

pub fn get_image<'a>(resource_name: &Name, data: &Arc<Vec<u8>>) -> egui::Image<'a> {
    egui::Image::new(
        <(String, egui::load::Bytes) as Into<egui::ImageSource>>::into((
            format!("bytes://{}.dds", resource_name),
            egui::load::Bytes::from(data.to_vec()),
        )),
    )
    .texture_options(egui::TextureOptions::NEAREST)
    .shrink_to_fit()
}
