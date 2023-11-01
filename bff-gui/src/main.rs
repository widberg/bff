#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

use artifact::Artifact;
use bff::bigfile::BigFile;
use bff::names::Name;
use helpers::load::load_bf;

pub mod artifact;
pub mod helpers;
mod panels;
pub mod traits;
mod views;

const TITLE: &str = "BFF Studio";
const WINDOW_SIZE: egui::Vec2 = egui::vec2(800.0, 600.0);

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().expect("Unable to create Runtime");

    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        })
    });

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        renderer: eframe::Renderer::Glow,
        icon_data: Some(
            eframe::IconData::try_from_png_bytes(include_bytes!("../resources/bff.png")).unwrap(),
        ),
        initial_window_size: Some(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(TITLE, options, Box::new(|cc| Box::new(Gui::new(cc))))
}

fn setup_custom_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "icons".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../resources/Font Awesome 6 Free-Solid-900.otf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("icons".into()))
        .or_default()
        .push("icons".to_owned());

    ctx.set_fonts(fonts);
}

#[derive(Default)]
enum GuiWindow {
    #[default]
    None,
    Rename,
}

struct Gui {
    open_window: GuiWindow,
    tx: Sender<Option<(BigFile, PathBuf)>>,
    rx: Receiver<Option<(BigFile, PathBuf)>>,
    bigfile: Option<BigFile>,
    bigfile_path: Option<PathBuf>,
    bigfile_loading: bool,
    resource_name: Option<Name>,
    nicknames: HashMap<Name, String>,
    nickname_editing: (Name, String),
    artifacts: HashMap<Name, Artifact>,
    infos: HashMap<Name, String>,
}

impl Gui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.25);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_custom_font(&cc.egui_ctx);
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            open_window: GuiWindow::default(),
            tx,
            rx,
            bigfile: None,
            bigfile_path: None,
            bigfile_loading: false,
            resource_name: None,
            nicknames: HashMap::new(),
            nickname_editing: (Name::default(), String::new()),
            artifacts: HashMap::new(),
            infos: HashMap::new(),
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(res) = self.rx.try_recv() {
            if let Some((bf, path)) = res {
                frame.set_window_title(format!("{} - {}", TITLE, path.to_string_lossy()).as_str());
                self.bigfile = Some(bf);
                self.bigfile_path = Some(path);
                self.nicknames.clear();
                self.resource_name = None;
            }
            self.bigfile_loading = false;
            ctx.set_cursor_icon(egui::CursorIcon::Default);
        }

        if self.bigfile_loading {
            ctx.set_cursor_icon(egui::CursorIcon::Progress);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                let menubar_reponse = self.menubar_panel(ui, frame, "menubar".into());
                if menubar_reponse.bf_loading {
                    self.bigfile_loading = true;
                }

                let resource_list_response = self.resource_list_panel(
                    ui,
                    format!(
                        "resources-{}",
                        self.bigfile_path
                            .as_ref()
                            .unwrap_or(&PathBuf::default())
                            .display()
                    )
                    .into(),
                );
                if let Some(name) = resource_list_response.resource_context_menu {
                    self.open_window = GuiWindow::Rename;
                    self.nickname_editing.0 = name;
                    if let Some(nn) = self.nicknames.get(&name) {
                        self.nickname_editing.1 = nn.clone();
                    }
                }
                if let Some(name) = resource_list_response.nickname_cleared {
                    self.nicknames.remove(&name);
                }
                if let Some(name) = resource_list_response.resource_clicked {
                    self.resource_name = Some(name);
                    if let Some(artifact) = resource_list_response.artifact_created {
                        self.artifacts.insert(name, artifact);
                    }
                    if let Some(info) = resource_list_response.info_created {
                        self.infos.insert(name, info);
                    }
                }

                self.resource_info_panel(ui);
                self.preview_panel(ui);

                match self.open_window {
                    GuiWindow::Rename => {
                        let mut is_open = true;
                        egui::Window::new("Change resource nickname")
                            .open(&mut is_open)
                            .fixed_size(egui::vec2(100.0, 50.0))
                            .show(ctx, |ui| {
                                ui.horizontal(|ui| {
                                    let output =
                                        egui::TextEdit::singleline(&mut self.nickname_editing.1)
                                            .hint_text("Enter nickname...")
                                            .min_size(ui.available_size())
                                            .show(ui);
                                    if (output.response.lost_focus()
                                        && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                                        || ui.button("Change").clicked()
                                    {
                                        let filtered_nickname = self.nickname_editing.1.trim();
                                        self.open_window = GuiWindow::None;
                                        if !filtered_nickname.is_empty() {
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
                        if !is_open {
                            self.open_window = GuiWindow::None;
                        }
                    }
                    GuiWindow::None => (),
                }
            });

        preview_files_being_dropped(ctx);

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let path = i.raw.dropped_files.get(0).unwrap().path.as_ref().unwrap();
                load_bf(ctx.clone(), path.clone(), self.tx.clone());
            }
        });
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use std::fmt::Write as _;

    use egui::*;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping BigFile:\n".to_owned();
            if let Some(file) = i.raw.hovered_files.first() {
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
