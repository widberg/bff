use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use bff::names::Name;

use crate::views::mesh::MeshView;
use crate::Artifact;

pub fn view(
    ui: &mut egui::Ui,
    id_source: egui::Id,
    resource_name: &Option<Name>,
    artifacts: &HashMap<Name, Artifact>,
) {
    // let mut response = ViewResponse::default();
    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(resource_name) = resource_name {
            let artifact = artifacts.get(resource_name);
            if let Some(a) = artifact {
                match a {
                    Artifact::Bitmap { is_dds: _, data } => {
                        ui.add(get_image(resource_name, data));
                    }
                    Artifact::Sound {
                        data,
                        sample_rate,
                        channels,
                    } => {
                        ui.horizontal(|ui| {
                            let mut volume = match ui
                                .memory(|mem| mem.data.get_temp::<Arc<Mutex<f32>>>(id_source))
                            {
                                Some(val) => *val.lock().unwrap(),
                                None => 1.0,
                            };
                            if ui
                                .button(
                                    egui::RichText::new("")
                                        .family(egui::FontFamily::Name("icons".into())),
                                )
                                .clicked()
                            {
                                play_sound(Arc::clone(data), *sample_rate, *channels, volume);
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
                    Artifact::Mesh(model) => {
                        ui.add(MeshView::new(Arc::clone(model)));
                    }
                    Artifact::Skin(skin) => {
                        ui.add(MeshView::new(Arc::clone(skin)));
                    }
                }
            }
        }
    });
}

fn get_image<'a>(resource_name: &Name, data: &Arc<Vec<u8>>) -> egui::Image<'a> {
    egui::Image::new(
        <(String, egui::load::Bytes) as Into<egui::ImageSource>>::into((
            format!("bytes://{}.dds", resource_name),
            egui::load::Bytes::from(data.to_vec()),
        )),
    )
    .texture_options(egui::TextureOptions::NEAREST)
    .shrink_to_fit()
}

fn play_sound(data: Arc<Vec<i16>>, sample_rate: u32, channels: u16, volume: f32) {
    std::thread::spawn(move || {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut bytes = Vec::new();
        let mut write_cursor = Cursor::new(&mut bytes);
        let mut parent_writer = hound::WavWriter::new(&mut write_cursor, spec).unwrap();
        let mut sample_writer = parent_writer.get_i16_writer(data.len() as u32);

        for sample in data.iter() {
            sample_writer.write_sample(*sample);
        }
        sample_writer.flush().unwrap();
        parent_writer.finalize().unwrap();

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let buf = std::io::BufReader::new(Cursor::new(bytes));
        let source = rodio::Decoder::new_wav(buf).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.sleep_until_end();
    });
}
