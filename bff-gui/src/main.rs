// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::{collections::HashMap, io::Write};

use bff::{
    bigfile::{resource::Resource, BigFile},
    class::Class,
    names::Name,
    platforms::Platform,
    traits::TryIntoVersionPlatform,
};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native("BFF GUI", options, Box::new(|_cc| Box::<Gui>::default()))
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> egui::Response {
    ui.add(egui::TextEdit::multiline(&mut text))
}

#[derive(Default)]
struct Gui {
    bigfile: Option<BigFile>,
    bigfile_path: Option<PathBuf>,
    resource_name: Option<Name>,
    nicknames: HashMap<Name, String>,
    nickname_window_open: bool,
    nickname_editing: (Name, String),
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.25);
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("top").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open BigFile...").clicked() {
                                ui.close_menu();
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter(
                                        "BigFile",
                                        &bff::platforms::extensions()
                                            .iter()
                                            .map(|s| s.to_str().unwrap())
                                            .collect::<Vec<&str>>()[..],
                                    )
                                    .pick_file()
                                {
                                    self.bigfile = Some(load_bigfile(&path));
                                    self.bigfile_path = Some(path);
                                    self.resource_name = None;
                                }
                            }
                        });
                        ui.menu_button("Export", |ui| {
                            if ui
                                .add_enabled(
                                    match self.resource_name {
                                        Some(_) => true,
                                        None => false,
                                    },
                                    egui::Button::new("Export JSON..."),
                                )
                                .clicked()
                            {
                                ui.close_menu();
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("json", &["json"])
                                    .save_file()
                                {
                                    File::create(path)
                                        .unwrap()
                                        .write_all(
                                            serde_json::to_string_pretty::<Class>(
                                                &self
                                                    .bigfile
                                                    .as_ref()
                                                    .unwrap()
                                                    .objects
                                                    .get(&self.resource_name.unwrap())
                                                    .unwrap()
                                                    .try_into_version_platform(
                                                        self.bigfile
                                                            .as_ref()
                                                            .unwrap()
                                                            .manifest
                                                            .version
                                                            .clone(),
                                                        self.bigfile
                                                            .as_ref()
                                                            .unwrap()
                                                            .manifest
                                                            .platform
                                                            .clone(),
                                                    )
                                                    .unwrap(),
                                            )
                                            .unwrap()
                                            .as_bytes(),
                                        )
                                        .unwrap();
                                }
                            }
                            if ui
                                .add_enabled(
                                    match self.resource_name {
                                        Some(_) => true,
                                        None => false,
                                    },
                                    egui::Button::new("Export data..."),
                                )
                                .clicked()
                            {
                                ui.close_menu();
                            }
                        });
                        ui.menu_button("Nicknames", |ui| {
                            if ui
                                .add_enabled(
                                    match self.bigfile {
                                        Some(_) => true,
                                        None => false,
                                    },
                                    egui::Button::new("Import..."),
                                )
                                .clicked()
                            {
                                ui.close_menu();
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("json", &["json"])
                                    .pick_file()
                                {
                                    let buf = std::io::BufReader::new(File::open(path).unwrap());
                                    self.nicknames = serde_json::de::from_reader::<
                                        std::io::BufReader<File>,
                                        HashMap<Name, String>,
                                    >(buf)
                                    .unwrap();
                                }
                            }

                            if ui
                                .add_enabled(
                                    !self.nicknames.is_empty(),
                                    egui::Button::new("Export all..."),
                                )
                                .clicked()
                            {
                                ui.close_menu();
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("json", &["json"])
                                    .set_file_name(format!(
                                        "{}_nicknames",
                                        self.bigfile_path
                                            .as_ref()
                                            .unwrap()
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                    ))
                                    .save_file()
                                {
                                    File::create(path)
                                        .unwrap()
                                        .write_all(
                                            serde_json::to_string_pretty(&self.nicknames)
                                                .unwrap()
                                                .as_bytes(),
                                        )
                                        .unwrap();
                                }
                            }
                            if ui
                                .add_enabled(
                                    !self.nicknames.is_empty(),
                                    egui::Button::new("Clear all"),
                                )
                                .clicked()
                            {
                                ui.close_menu();
                                self.nicknames.clear();
                            }
                        });
                    });
                });

                egui::SidePanel::left("left")
                    .resizable(true)
                    .width_range(70.0..=ui.available_width() / 2.0)
                    .show_inside(ui, |ui| {
                        // ui.set_width_range(150.0..=200.0);
                        if let Some(bigfile) = &self.bigfile {
                            let resources: Vec<&Resource> = bigfile.objects.values().collect();
                            let row_height = ui.spacing().interact_size.y;
                            egui::ScrollArea::vertical().show_rows(
                                ui,
                                row_height,
                                resources.len(),
                                |ui, row_range| {
                                    ui.set_min_width(ui.available_width());
                                    for row in row_range {
                                        let resource = resources.get(row).unwrap();
                                        if ui
                                            .add(
                                                egui::Button::new(format!(
                                                    "{}.{}",
                                                    match self.nicknames.get(&resource.name) {
                                                        Some(nn) => nn.to_owned(),
                                                        None => resource.name.to_string(),
                                                    },
                                                    resource.class_name.to_string()
                                                ))
                                                .wrap(false)
                                                .rounding(0.0)
                                                .min_size(egui::vec2(ui.available_width(), 0.0)),
                                            )
                                            .context_menu(|ui| {
                                                if ui.button("Change nickname").clicked() {
                                                    self.nickname_window_open = true;
                                                    self.nickname_editing.0 = resource.name;
                                                    ui.close_menu();
                                                }
                                            })
                                            .clicked()
                                        {
                                            self.resource_name = Some(resource.name);
                                        }
                                    }
                                },
                            );
                        }
                    });
                egui::SidePanel::right("right")
                    .resizable(true)
                    .width_range(100.0..=ui.available_width() / 2.0)
                    .show_inside(ui, |ui| {
                        if let Some(resource_name) = &self.resource_name {
                            let j = serde_json::to_string_pretty::<Class>(
                                &self
                                    .bigfile
                                    .as_ref()
                                    .unwrap()
                                    .objects
                                    .get(resource_name)
                                    .unwrap()
                                    .try_into_version_platform(
                                        self.bigfile.as_ref().unwrap().manifest.version.clone(),
                                        self.bigfile.as_ref().unwrap().manifest.platform.clone(),
                                    )
                                    .unwrap(),
                            )
                            .unwrap();
                            let json: Vec<&str> = j
                                .split_inclusive("\n")
                                // .map(|s| s.to_string())
                                .collect();
                            let text_style = egui::TextStyle::Body;
                            egui::ScrollArea::vertical()
                                .id_source("code_scroll")
                                .show_rows(
                                    ui,
                                    ui.text_style_height(&text_style),
                                    json.len(),
                                    |ui, row_range| {
                                        let content: String = row_range
                                            .into_iter()
                                            .map(|i| *json.get(i).unwrap())
                                            .collect();
                                        ui.set_min_width(ui.available_width());
                                        selectable_text(ui, content.as_str())
                                    },
                                );
                        }
                    });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if let Some(resource_name) = &self.resource_name {
                        let class: Class = self
                            .bigfile
                            .as_ref()
                            .unwrap()
                            .objects
                            .get(resource_name)
                            .unwrap()
                            .try_into_version_platform(
                                self.bigfile.as_ref().unwrap().manifest.version.clone(),
                                self.bigfile.as_ref().unwrap().manifest.platform.clone(),
                            )
                            .unwrap();
                        match class {
                            Class::Bitmap(box_bitmap) => match *box_bitmap {
                                bff::class::bitmap::Bitmap::BitmapV1_291_03_06PC(ref bitmap) => {
                                    ui.add(get_image(resource_name, &bitmap.body.data));
                                }
                                bff::class::bitmap::Bitmap::BitmapV1_381_67_09PC(ref bitmap) => {
                                    ui.add(get_image(resource_name, &bitmap.body.data));
                                }
                                _ => (),
                            },
                            Class::Sound(box_sound) => match *box_sound {
                                bff::class::sound::Sound::SoundV1_291_03_06PC(sound) => {
                                    if ui.button("play").clicked() {
                                        play_sound(
                                            sound.body.flags.stereo().value(),
                                            sound.body.sample_rate,
                                            sound.body.data,
                                        );
                                    }
                                }
                                bff::class::sound::Sound::SoundV1_381_67_09PC(sound) => {
                                    if ui.button("play").clicked() {
                                        play_sound(
                                            sound.link_header.flags.stereo().value(),
                                            sound.link_header.sample_rate,
                                            sound.body.data,
                                        );
                                    }
                                }
                            },
                            _ => (),
                        }
                    }
                });

                if self.nickname_window_open {
                    egui::Window::new("Change resource nickname")
                        .fixed_size(egui::vec2(100.0, 50.0))
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                let output =
                                    egui::TextEdit::singleline(&mut self.nickname_editing.1)
                                        .hint_text("Enter nickname...")
                                        .min_size(egui::vec2(100.0, 0.0))
                                        .show(ui);
                                if (output.response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                                    || ui.button("Change").clicked()
                                {
                                    let filtered_nickname = self.nickname_editing.1.trim();
                                    self.nickname_window_open = false;
                                    if filtered_nickname.len() != 0 {
                                        self.nicknames.insert(
                                            self.nickname_editing.0,
                                            filtered_nickname.to_owned(),
                                        );
                                    } else {
                                        self.nicknames.remove(&self.nickname_editing.0);
                                    }
                                    self.nickname_editing.1 = String::new();
                                }
                            });
                        });
                }
            });

        preview_files_being_dropped(ctx);

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.bigfile = Some(load_bigfile(
                    i.raw.dropped_files.get(0).unwrap().path.as_ref().unwrap(),
                ))
            }
        });
    }
}

fn load_bigfile(path: &PathBuf) -> BigFile {
    let platform = match path.extension() {
        Some(extension) => extension.try_into().unwrap_or(Platform::PC),
        None => Platform::PC,
    };
    let f = File::open(path).unwrap();
    let mut reader = bff::BufReader::new(f);
    BigFile::read_platform(&mut reader, platform).unwrap()
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

fn play_sound(stereo: u8, sample_rate: u32, data: Vec<i16>) {
    std::thread::spawn(move || {
        let spec = hound::WavSpec {
            channels: match stereo {
                1 => 2,
                _ => 1,
            },
            sample_rate: sample_rate,
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
        sink.append(source);
        sink.sleep_until_end();
    });
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
