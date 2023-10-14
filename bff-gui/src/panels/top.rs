use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;
use egui;
use image::EncodableLayout;

use crate::load_bigfile;

#[derive(Default)]
pub struct MenubarResponse {
    pub bigfile_open: Option<(BigFile, PathBuf)>,
}

pub fn menubar(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    bigfile: &Option<BigFile>,
    bigfile_path: &Option<PathBuf>,
    resource_name: &Option<Name>,
    nicknames: &mut HashMap<Name, String>,
) -> MenubarResponse {
    let mut response = MenubarResponse::default();
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
                        response.bigfile_open = Some((load_bigfile(&path), path));
                    }
                }
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });
            ui.menu_button("Export", |ui| {
                if ui
                    .add_enabled(resource_name.is_some(), egui::Button::new("Export JSON..."))
                    .clicked()
                {
                    ui.close_menu();
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .save_file()
                    {
                        write_class(
                            &path,
                            bigfile.as_ref().unwrap(),
                            resource_name.as_ref().unwrap_or(&Name::default()),
                        )
                    }
                }
                if ui
                    .add_enabled(false, egui::Button::new("Export data..."))
                    .clicked()
                {
                    ui.close_menu();
                }
                if ui
                    .add_enabled(resource_name.is_some(), egui::Button::new("Export raw..."))
                    .clicked()
                {
                    ui.close_menu();
                    let resource = bigfile
                        .as_ref()
                        .unwrap()
                        .objects
                        .get(&resource_name.unwrap())
                        .unwrap();
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("raw", &[resource.class_name.to_string()])
                        .save_file()
                    {
                        let data = match &resource.data {
                            bff::bigfile::resource::ResourceData::Data(data) => data.clone(),
                            bff::bigfile::resource::ResourceData::CompressibleData {
                                data, ..
                            } => data.clone(),
                            bff::bigfile::resource::ResourceData::ExtendedData {
                                link_header,
                                body,
                                ..
                            } => [link_header.clone(), body.clone()].concat(),
                        };
                        File::create(path)
                            .unwrap()
                            .write_all(data.as_bytes())
                            .unwrap();
                    }
                }
            });
            ui.menu_button("Nicknames", |ui| {
                if ui
                    .add_enabled(bigfile.is_some(), egui::Button::new("Import..."))
                    .clicked()
                {
                    ui.close_menu();
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .pick_file()
                    {
                        let buf = std::io::BufReader::new(File::open(path).unwrap());
                        *nicknames = serde_json::de::from_reader::<
                            std::io::BufReader<File>,
                            HashMap<Name, String>,
                        >(buf)
                        .unwrap();
                    }
                }

                if ui
                    .add_enabled(!nicknames.is_empty(), egui::Button::new("Export all..."))
                    .clicked()
                {
                    ui.close_menu();
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_file_name(format!(
                            "{}_nicknames",
                            bigfile_path
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
                            .write_all(serde_json::to_string_pretty(nicknames).unwrap().as_bytes())
                            .unwrap();
                    }
                }
                if ui
                    .add_enabled(!nicknames.is_empty(), egui::Button::new("Clear all"))
                    .clicked()
                {
                    ui.close_menu();
                    nicknames.clear();
                }
            });
        });
    });
    response
}

fn write_class(path: &PathBuf, bigfile: &BigFile, resource_name: &Name) {
    File::create(path)
        .unwrap()
        .write_all(
            serde_json::to_string_pretty::<Class>(
                &bigfile
                    .objects
                    .get(resource_name)
                    .unwrap()
                    .try_into_version_platform(
                        bigfile.manifest.version.clone(),
                        bigfile.manifest.platform,
                    )
                    .unwrap(),
            )
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
}
