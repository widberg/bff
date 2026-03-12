use std::f64;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use rodio::Source;

use crate::helpers::sound::play_sound;

#[derive(Clone)]
struct SoundPreview {
    duration: Option<f64>,
    waveform: Arc<[f32]>,
}

fn decode_preview(data: &Arc<[u8]>, target_points: usize) -> Option<SoundPreview> {
    let decoder = rodio::Decoder::new_wav(Cursor::new(data.clone())).ok()?;
    let duration = decoder.total_duration().map(|value| value.as_secs_f64());
    let channels = usize::from(decoder.channels().get()).max(1);
    let samples: Vec<f32> = decoder.collect();

    if samples.is_empty() {
        return Some(SoundPreview {
            duration,
            waveform: Arc::from(Vec::<f32>::new()),
        });
    }

    let mut mono = Vec::with_capacity(samples.len() / channels + 1);
    for frame in samples.chunks(channels) {
        let avg = frame.iter().copied().sum::<f32>() / frame.len() as f32;
        mono.push(avg.clamp(-1.0, 1.0));
    }

    let stride = (mono.len() / target_points.max(1)).max(1);
    let waveform: Vec<f32> = mono.into_iter().step_by(stride).collect();

    Some(SoundPreview {
        duration,
        waveform: Arc::from(waveform),
    })
}

pub fn sound_view(ui: &mut egui::Ui, id_source: egui::Id, data: Arc<[u8]>) {
    let preview = match ui.memory(|mem| mem.data.get_temp::<Arc<SoundPreview>>(id_source)) {
        Some(preview) => preview,
        None => {
            let preview = Arc::new(decode_preview(&data, 2048).unwrap_or(SoundPreview {
                duration: None,
                waveform: Arc::from(Vec::<f32>::new()),
            }));
            ui.memory_mut(|mem| mem.data.insert_temp(id_source, preview.clone()));
            preview
        }
    };

    let mut volume = match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<f32>>>(id_source)) {
        Some(val) => *val.lock().unwrap(),
        None => 1.0,
    };
    let time = match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<f64>>>(id_source)) {
        Some(val) => *val.lock().unwrap(),
        None => f64::NAN,
    };
    let duration = preview.duration;
    let elapsed = ui.input(|i| i.time) - time;
    let size = ui.available_size();

    ui.horizontal_wrapped(|ui| {
        if ui
            .button(egui::RichText::new("").family(egui::FontFamily::Name("icons".into())))
            .clicked()
        {
            play_sound(data.clone(), volume);
            let t = ui.input(|i| i.time);
            ui.memory_mut(|mem| mem.data.insert_temp(id_source, Arc::new(Mutex::new(t))));
        }
        match duration {
            Some(duration) => ui.label(format!(
                "{:.2}/{:.2}",
                if time.is_nan() {
                    0.0
                } else {
                    elapsed.min(duration)
                },
                duration
            )),
            None => ui.label(format!(
                "{:.2}/--",
                if time.is_nan() { 0.0 } else { elapsed.max(0.0) }
            )),
        };

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

        if preview.waveform.len() > 1 {
            let points: Vec<egui::Pos2> = preview
                .waveform
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let t = i as f32 / (preview.waveform.len() - 1) as f32;
                    to_screen * egui::pos2(t, *v)
                })
                .collect();
            ui.painter().add(egui::epaint::Shape::line(
                points,
                egui::epaint::PathStroke::new(1.0, egui::Color32::WHITE),
            ));
        } else {
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "No waveform data",
                egui::FontId::default(),
                egui::Color32::GRAY,
            );
        }

        if let Some(duration) = duration
            && !time.is_nan()
            && duration > 0.0
        {
            ui.ctx().request_repaint();

            if elapsed >= duration {
                ui.memory_mut(|mem| {
                    mem.data
                        .insert_temp(id_source, Arc::new(Mutex::new(f64::NAN)))
                });
            } else {
                let x = (elapsed / duration).clamp(0.0, 1.0) as f32;
                ui.painter().add(egui::epaint::Shape::line_segment(
                    [
                        to_screen * egui::pos2(x, -1.0),
                        to_screen * egui::pos2(x, 1.0),
                    ],
                    egui::epaint::Stroke::new(2.0, egui::Color32::GRAY),
                ));
            }
        }
    });
}
