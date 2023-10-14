use std::collections::HashMap;
use std::io::Cursor;

use bff::names::Name;

use crate::views::mesh::MeshView;
use crate::Artifact;

pub fn view(
    ui: &mut egui::Ui,
    resource_name: &Option<Name>,
    artifacts: &HashMap<Name, Artifact>,
    mut sound_volume: f32,
) {
    // let mut response = ViewResponse::default();
    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(resource_name) = resource_name {
            let artifact = artifacts.get(resource_name);
            if let Some(a) = artifact {
                match a {
                    Artifact::Bitmap(bitmap) => {
                        ui.add(get_image(resource_name, bitmap));
                    }
                    Artifact::Sound {
                        data,
                        sample_rate,
                        channels,
                    } => {
                        ui.add(
                            egui::Slider::new(&mut sound_volume, 0.0..=1.0)
                                .text("Volume")
                                .show_value(false),
                        );
                        if ui.button("play").clicked() {
                            play_sound(data.clone(), *sample_rate, *channels, sound_volume);
                        }
                    }
                    Artifact::Mesh(model) => {
                        ui.add(MeshView::new(model.clone()));
                    }
                }
            }
        }
    });
}

fn get_image<'a>(resource_name: &Name, data: &Vec<u8>) -> egui::Image<'a> {
    egui::Image::new(
        <(String, std::vec::Vec<u8>) as Into<egui::ImageSource>>::into((
            format!("bytes://{}.dds", resource_name),
            data.to_owned(),
        )),
    )
    .texture_options(egui::TextureOptions::NEAREST)
    .shrink_to_fit()
}

fn play_sound(data: Vec<i16>, sample_rate: u32, channels: u16, volume: f32) {
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

        for sample in data {
            sample_writer.write_sample(sample);
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
