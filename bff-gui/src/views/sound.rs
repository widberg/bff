use std::sync::{Arc, Mutex};

use crate::helpers::sound::play_sound;

pub fn sound_view(
    ui: &mut egui::Ui,
    id_source: egui::Id,
    data: Arc<Vec<i16>>,
    sample_rate: u32,
    channels: u16,
) {
    ui.horizontal(|ui| {
        let mut volume = match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<f32>>>(id_source)) {
            Some(val) => *val.lock().unwrap(),
            None => 1.0,
        };
        if ui
            .button(egui::RichText::new("").family(egui::FontFamily::Name("icons".into())))
            .clicked()
        {
            play_sound(data, sample_rate, channels, volume);
        }
        let response = ui.add(
            egui::Slider::new(&mut volume, 0.0..=1.0)
                .text("Volume")
                .show_value(false),
        );
        if response.changed() {
            ui.memory_mut(|mem| {
                mem.data
                    .insert_temp(id_source, Arc::new(Mutex::new(volume)))
            });
        }
    });
}
