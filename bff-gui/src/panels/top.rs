#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use bff::names::Name;

use crate::Gui;
#[cfg(not(target_arch = "wasm32"))]
use crate::artifact::Artifact;
#[cfg(not(target_arch = "wasm32"))]
use crate::helpers::class::write_class_json;
use crate::helpers::load::load_bf;

#[cfg(not(target_arch = "wasm32"))]
fn load_names(in_name: &PathBuf) {
    if let Ok(f) = File::open(in_name) {
        let mut reader = bff::BufReader::new(f);
        bff::names::names()
            .lock()
            .unwrap()
            .read(&mut reader)
            .unwrap();
    }
}

impl Gui {
    #[cfg(not(target_arch = "wasm32"))]
    fn create_file_dialog(&mut self, ui: &mut egui::Ui, names_checked: bool) -> bool {
        let dialog = bff::bigfile::platforms::extensions()
            .iter()
            .map(|s| s.to_str().unwrap())
            .fold(
                rfd::FileDialog::new().add_filter("All files", &["*"]),
                |acc, d| acc.add_filter(d, &[d]),
            );
        if let Some(path) = dialog.pick_file() {
            if names_checked {
                let mut extension = path.extension().unwrap().to_str().unwrap().to_owned();
                extension.replace_range(..1, "N");
                let in_name = path.with_extension(extension);
                load_names(&in_name);
            }
            load_bf(ui.ctx().clone(), path, self.tx.clone());
            return true;
        }
        false
    }
    #[cfg(target_arch = "wasm32")]
    fn create_file_dialog(&mut self, ui: &mut egui::Ui, _names: bool) -> bool {
        let ctx = ui.ctx().clone();
        let tx = self.tx.clone();
        let future = async {
            let dialog = bff::bigfile::platforms::extensions()
                .iter()
                .map(|s| s.to_str().unwrap())
                .fold(
                    rfd::AsyncFileDialog::new().add_filter("All files", &["*"]),
                    |acc, d| acc.add_filter(d, &[d]),
                )
                .pick_file()
                .await
                .unwrap();
            load_bf(ctx, dialog.file_name(), dialog.read().await, tx);
        };
        async_std::task::block_on(future);
        true
    }
    pub fn menubar_panel(
        &mut self,
        ctx: &egui::Context,
        #[allow(unused_variables)] frame: &mut eframe::Frame,
        id_source: egui::Id,
    ) -> bool {
        let mut response = false;
        egui::TopBottomPanel::top("top").show(ctx, |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    ui.menu_button("BigFile", |ui| {
                        #[allow(unused_mut)]
                        let mut checked = match ui
                            .memory(|mem| mem.data.get_temp::<Arc<Mutex<bool>>>(id_source))
                        {
                            Some(val) => *val.lock().unwrap(),
                            None => true,
                        };
                        if ui.button("Open BigFile...").clicked() {
                            ui.close_menu();
                            response = self.create_file_dialog(ui, checked);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let names_cbox = ui
                                .checkbox(&mut checked, "Auto-load names")
                                .on_hover_text_at_pointer(
                                "If the parent directory contains an associated name file, load it",
                            );
                            if names_cbox.changed() {
                                ui.memory_mut(|mem| {
                                    mem.data
                                        .insert_temp(id_source, Arc::new(Mutex::new(checked)))
                                });
                            }
                        }
                    });
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let load_names_btn = ui.add_enabled(
                            self.bigfile.is_some(),
                            egui::Button::new("Load names..."),
                        );
                        if load_names_btn.clicked() {
                            let dialog = bff::bigfile::platforms::extensions()
                                .iter()
                                .map(|s| s.to_str().unwrap().replacen('D', "N", 1))
                                .filter(|s| !s.contains("BF")) //TODO: actual name files for everything
                                .fold(
                                    rfd::FileDialog::new().add_filter("All files", &["*"]),
                                    |acc, d| acc.add_filter(d.clone(), &[d.as_str()]),
                                );
                            if let Some(paths) = dialog.pick_files() {
                                ui.close_menu();
                                paths.iter().for_each(load_names);
                            }
                        }
                        if ui.button("Quit").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("Export", |ui| {
                    ui.menu_button("Selected", |ui| {
                        if ui
                            .add_enabled(
                                self.resource_name.is_some(),
                                egui::Button::new("Export JSON..."),
                            )
                            .clicked()
                        {
                            ui.close_menu();
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("json", &["json"])
                                .save_file()
                            {
                                write_class_json(
                                    &path,
                                    self.bigfile.as_ref().unwrap(),
                                    self.resource_name.as_ref().unwrap_or(&Name::default()),
                                )
                            }
                        }
                        let rich_enabled = match self.resource_name {
                            Some(n) => self.artifacts.contains_key(&n),
                            None => false,
                        };
                        if ui
                            .add_enabled(rich_enabled, egui::Button::new("Export rich..."))
                            .clicked()
                        {
                            ui.close_menu();
                            let artifact =
                                self.artifacts.get(&self.resource_name.unwrap()).unwrap();
                            let (ext_name, exts) = match artifact {
                                Artifact::Bitmap { .. } => ("image", vec!["png", "dds"]),
                                Artifact::Sound { .. } => ("sound", vec!["wav"]),
                                Artifact::Mesh(..) | Artifact::Skin(..) => ("mesh", vec!["glb"]),
                            };
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter(ext_name, &exts)
                                .save_file()
                            {
                                artifact.save(&path).unwrap();
                            }
                        }
                        if ui
                            .add_enabled(
                                self.resource_name.is_some(),
                                egui::Button::new("Export binary..."),
                            )
                            .clicked()
                        {
                            ui.close_menu();
                            let resource = self
                                .bigfile
                                .as_ref()
                                .unwrap()
                                .resources
                                .get(&self.resource_name.unwrap())
                                .unwrap();
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("raw", &[resource.class_name.to_string()])
                                    .save_file()
                                {
                                    let mut w = File::create(path).unwrap();
                                    self.bigfile
                                        .as_ref()
                                        .unwrap()
                                        .dump_resource(resource, &mut w)
                                        .unwrap();
                                }
                            }
                            // FIXME: This nested cfg will never be active
                            #[cfg(target_arch = "wasm32")]
                            async {
                                let mut w: std::io::Cursor<Vec<u8>> =
                                    std::io::Cursor::new(Vec::new());
                                self.bigfile
                                    .as_ref()
                                    .unwrap()
                                    .dump_resource(resource, &mut w)
                                    .unwrap();
                                rfd::AsyncFileDialog::new()
                                    .add_filter("raw", &[resource.class_name.to_string()])
                                    .save_file()
                                    .await
                                    .unwrap()
                                    .write(&w.into_inner());
                            };
                        }
                    });
                    ui.menu_button("All", |ui| {
                        if ui
                            .add_enabled(
                                self.bigfile.is_some(),
                                egui::Button::new("Export binary..."),
                            )
                            .clicked()
                        {
                            ui.close_menu();
                            if let Some(directory) = rfd::FileDialog::new().pick_folder() {
                                for resource in self.bigfile.as_ref().unwrap().resources.values() {
                                    let name = resource.name;
                                    let class_name = resource.class_name;
                                    let path = directory.join(
                                        format!("{}.{}", name, class_name)
                                            .replace(':', "_")
                                            .replace('>', "-"),
                                        // works fine but i don't really like this solution
                                    );
                                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                                    let mut writer =
                                        std::io::BufWriter::new(File::create(path).unwrap());
                                    self.bigfile
                                        .as_ref()
                                        .unwrap()
                                        .dump_resource(resource, &mut writer)
                                        .unwrap();
                                }
                            }
                        }
                    });
                });
                // TODO: lol
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("Nicknames", |ui| {
                    if ui
                        .add_enabled(false, egui::Button::new("Import...")) // TODO: Check if bigfile.is_some()
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
                        .add_enabled(!self.nicknames.is_empty(), egui::Button::new("Clear all"))
                        .clicked()
                    {
                        ui.close_menu();
                        self.nicknames.clear();
                    }
                });
                ui.add_enabled_ui(self.bigfile.is_some(), |ui| {
                    ui.menu_button("Info", |ui| {
                        ui.label(format!(
                            "Version: {}\nPlatform: {}",
                            self.bigfile.as_ref().unwrap().manifest.version,
                            self.bigfile.as_ref().unwrap().manifest.platform,
                        ))
                    })
                });
            });
        });
        response
    }
}
