use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;
use egui;

use crate::{load_bigfile, Artifact};

#[derive(Default)]
pub struct MenubarResponse {
    pub bigfile_open: Option<(BigFile, PathBuf)>,
}

pub fn menubar(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    id_source: egui::Id,
    bigfile: &Option<BigFile>,
    bigfile_path: &Option<PathBuf>,
    resource_name: &Option<Name>,
    nicknames: &mut HashMap<Name, String>,
    artifacts: &HashMap<Name, Artifact>,
) -> MenubarResponse {
    let mut response = MenubarResponse::default();
    egui::TopBottomPanel::top("top").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                ui.menu_button("BigFile", |ui| {
                    let mut checked =
                        match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<bool>>>(id_source)) {
                            Some(val) => *val.lock().unwrap(),
                            None => true,
                        };
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
                            if checked {
                                if let Some(extension) = path.extension() {
                                    if let Some(extension) = extension.to_str() {
                                        let mut extension = extension.to_string();
                                        extension.replace_range(..1, "N");
                                        let in_name = path.with_extension(extension);

                                        if let Ok(f) = File::open(in_name) {
                                            let mut reader = bff::BufReader::new(f);
                                            bff::names::names()
                                                .lock()
                                                .unwrap()
                                                .read(&mut reader)
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                            response.bigfile_open = Some((load_bigfile(&path), path));
                        }
                    }
                    if ui
                        .checkbox(&mut checked, "Auto-load names")
                        .on_hover_text_at_pointer(
                            "If the parent directory contains an associated name file, load it",
                        )
                        .changed()
                    {
                        ui.memory_mut(|mem| {
                            mem.data
                                .insert_temp(id_source, Arc::new(Mutex::new(checked)))
                        });
                    }
                });
                if ui
                    .add_enabled(bigfile.is_some(), egui::Button::new("Load names..."))
                    .clicked()
                {
                    if let Some(paths) = rfd::FileDialog::new()
                        .add_filter(
                            "Name files",
                            &bff::platforms::extensions()
                                .iter()
                                .map(|s| s.to_str().unwrap().replacen("D", "N", 1))
                                .collect::<Vec<String>>()[..],
                        )
                        .pick_files()
                    {
                        ui.close_menu();
                        for in_name in paths {
                            let f = File::open(in_name).unwrap();
                            let mut reader = bff::BufReader::new(f);
                            bff::names::names()
                                .lock()
                                .unwrap()
                                .read(&mut reader)
                                .unwrap();
                        }
                    }
                }
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });
            ui.menu_button("Export", |ui| {
                ui.menu_button("Selected", |ui| {
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
                    let rich_enabled = match resource_name {
                        Some(n) => artifacts.contains_key(n),
                        None => false,
                    };
                    if ui
                        .add_enabled(rich_enabled, egui::Button::new("Export rich..."))
                        .clicked()
                    {
                        ui.close_menu();
                        let artifact = artifacts.get(&resource_name.unwrap()).unwrap();
                        let extension = match artifact {
                            Artifact::Bitmap { .. } => "dds",
                            Artifact::Sound { .. } => "wav",
                            Artifact::Mesh(..) | Artifact::Skin(..) => "glb",
                        };
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter(extension, &[extension])
                            .save_file()
                        {
                            artifact.save(&path);
                        }
                    }
                    if ui
                        .add_enabled(
                            resource_name.is_some(),
                            egui::Button::new("Export binary..."),
                        )
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
                            let mut w = File::create(path).unwrap();
                            bigfile
                                .as_ref()
                                .unwrap()
                                .dump_resource(resource, &mut w)
                                .unwrap();
                        }
                    }
                });
                ui.menu_button("All", |ui| {
                    if ui
                        .add_enabled(bigfile.is_some(), egui::Button::new("Export binary..."))
                        .clicked()
                    {
                        ui.close_menu();
                        if let Some(directory) = rfd::FileDialog::new().pick_folder() {
                            for resource in bigfile.as_ref().unwrap().objects.values() {
                                let name = resource.name;
                                let class_name = resource.class_name;
                                let path = directory.join(
                                    format!("{}.{}", name, class_name)
                                        .replace(":", "_")
                                        .replace(">", "-"),
                                    // works fine but i don't really like this solution
                                );
                                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                                let mut writer =
                                    std::io::BufWriter::new(File::create(path).unwrap());
                                bigfile
                                    .as_ref()
                                    .unwrap()
                                    .dump_resource(resource, &mut writer)
                                    .unwrap();
                            }
                        }
                    }
                });
            });
            ui.menu_button("Nicknames", |ui| {
                if ui
                    .add_enabled(false, egui::Button::new("Import...")) // bigfile.is_some()
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
