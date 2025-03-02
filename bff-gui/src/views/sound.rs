use std::{
    f64,
    sync::{Arc, Mutex},
};

use crate::helpers::sound::play_sound;

pub fn sound_view(
    ui: &mut egui::Ui,
    id_source: egui::Id,
    data: Arc<Vec<i16>>,
    sample_rate: u32,
    channels: u16,
) {
    let mut volume = match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<f32>>>(id_source)) {
        Some(val) => *val.lock().unwrap(),
        None => 1.0,
    };
    let time = match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<f64>>>(id_source)) {
        Some(val) => *val.lock().unwrap(),
        None => f64::NAN,
    };
    let duration = data.len() as f64 / sample_rate as f64;
    let elapsed = ui.input(|i| i.time) - time;
    let size = ui.available_size();
    ui.horizontal_wrapped(|ui| {
        if ui
            .button(egui::RichText::new("").family(egui::FontFamily::Name("icons".into())))
            .clicked()
        {
            play_sound(data.clone(), sample_rate, channels, volume);
            let t = ui.input(|i| i.time);
            ui.memory_mut(|mem| mem.data.insert_temp(id_source, Arc::new(Mutex::new(t))));
        }
        ui.label(format!(
            "{:.2}/{:.2}",
            if time.is_nan() { 0.0 } else { elapsed },
            duration
        ));
        let response = ui.add(
            egui::Slider::new(&mut volume, 0.0..=1.0)
                .text("Volume")
                .text_color(egui::Color32::GRAY)
                .fixed_decimals(2)
                .show_value(true),
        );
        if response.changed() {
            ui.memory_mut(|mem| {
                mem.data
                    .insert_temp(id_source, Arc::new(Mutex::new(volume)))
            });
        }
    });

    egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
        let desired_size = size.x * egui::vec2(1.0, 0.35);
        let (_id, rect) = ui.allocate_space(desired_size);

        let to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
            rect,
        );

        let mut shapes = vec![];

        let points: Vec<egui::Pos2> = data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if i % 10 == 0 {
                    let t = i as f32 / data.len() as f32;
                    Some(to_screen * egui::pos2(t, *v as f32 / i16::MAX as f32))
                } else {
                    None
                }
            })
            .collect();

        let thickness = 1.0;
        shapes.push(egui::epaint::Shape::line(
            points,
            egui::epaint::PathStroke::new(thickness, egui::Color32::WHITE),
        ));
        ui.painter().extend(shapes);

        if !time.is_nan() {
            ui.ctx().request_repaint();

            if elapsed >= duration {
                ui.memory_mut(|mem| {
                    mem.data
                        .insert_temp(id_source, Arc::new(Mutex::new(f64::NAN)))
                });
            } else {
                ui.painter().add(egui::epaint::Shape::line_segment(
                    [
                        to_screen * egui::pos2((elapsed / duration) as f32, -1.0),
                        to_screen * egui::pos2((elapsed / duration) as f32, 1.0),
                    ],
                    egui::epaint::PathStroke::new(2.0, egui::Color32::GRAY),
                ));
            }
        }
    });
}
