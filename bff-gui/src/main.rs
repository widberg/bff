#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

use artifact::Artifact;
use bff::bigfile::BigFile;
use bff::names::Name;
#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use error::BffGuiResult;
#[cfg(not(target_arch = "wasm32"))]
use helpers::load::load_bf;

pub mod artifact;
pub mod error;
pub mod helpers;
mod panels;
pub mod traits;
mod views;

#[cfg(not(target_arch = "wasm32"))]
const TITLE: &str = "BFF Studio";
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_SIZE: egui::Vec2 = egui::vec2(800.0, 600.0);

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(group = clap::ArgGroup::new("one").multiple(false))]
struct Args {
    #[clap(group = "one")]
    file: Option<PathBuf>,
    #[cfg(target_os = "windows")]
    #[clap(long, group = "one")]
    install: bool,
    #[cfg(target_os = "windows")]
    #[clap(long, group = "one")]
    uninstall: bool,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> BffGuiResult<()> {
    use std::sync::Arc;

    let cli = Args::parse();
    let file = cli.file.clone();

    #[cfg(target_os = "windows")]
    {
        if cli.install {
            return install();
        } else if cli.uninstall {
            return uninstall();
        }
    }

    let rt = tokio::runtime::Runtime::new().expect("Unable to create Runtime");

    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        })
    });
    let viewport = egui::ViewportBuilder {
        drag_and_drop: Some(true),
        icon: Some(Arc::new(
            eframe::icon_data::from_png_bytes(include_bytes!("../resources/bff.png")).unwrap(),
        )),
        inner_size: Some(WINDOW_SIZE),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        TITLE,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            setup_custom_font(&cc.egui_ctx);
            cc.egui_ctx.set_pixels_per_point(1.25);
            Ok(Box::new(Gui::new(cc, file)))
        }),
    )?;

    Ok(())
}

#[cfg(target_os = "windows")]
const PROG_ID: &str = "Widberg.BFF.1";

#[cfg(target_os = "windows")]
fn change_notify() {
    use windows::Win32::UI::Shell::{SHCNE_ASSOCCHANGED, SHCNF_IDLIST, SHChangeNotify};

    unsafe {
        SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    }
}

#[cfg(target_os = "windows")]
fn install() -> BffGuiResult<()> {
    use std::env::current_exe;

    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let exe_path = current_exe()?.to_str().unwrap_or_default().to_owned();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey("Software\\Classes")?;

    let (prog, _) = classes.create_subkey(PROG_ID)?;
    prog.set_value("", &TITLE)?;
    let (command, _) = prog.create_subkey("Shell\\Open\\Command")?;
    command.set_value("", &format!(r#""{}" "%1""#, exe_path))?;

    for extension in bff::bigfile::platforms::extensions() {
        let (extension_key, _) =
            classes.create_subkey(format!(".{}", extension.to_string_lossy()))?;
        extension_key.set_value("", &PROG_ID)?;
        let (open_with, _) = extension_key.create_subkey("OpenWithProgids")?;
        open_with.set_value(PROG_ID, &"")?;
    }

    change_notify();

    Ok(())
}

#[cfg(target_os = "windows")]
fn uninstall() -> BffGuiResult<()> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey("Software\\Classes")?;

    let _ = classes.delete_subkey_all(PROG_ID);

    for extension in bff::bigfile::platforms::extensions() {
        if let Ok(default) = classes
            .open_subkey_with_flags(format!(".{}", extension.to_string_lossy()), KEY_ALL_ACCESS)
        {
            if let Ok(prog_id) = default.get_value::<String, _>("") {
                if prog_id == PROG_ID {
                    default.delete_value("")?;
                }
            }

            if let Ok(open_with) = default.open_subkey_with_flags("OpenWithProgids", KEY_ALL_ACCESS)
            {
                let _ = open_with.delete_value(PROG_ID);
            }
        }
    }

    change_notify();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use web_sys::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    setup_custom_font(&cc.egui_ctx);
                    cc.egui_ctx.set_pixels_per_point(1.25);
                    Ok(Box::new(Gui::new(cc)))
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
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
    fn new(
        #[allow(unused_variables)] cc: &eframe::CreationContext<'_>,
        #[cfg(not(target_arch = "wasm32"))] file: Option<PathBuf>,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        #[cfg(not(target_arch = "wasm32"))]
        let bf_loading = match file {
            Some(path) => {
                load_bf(cc.egui_ctx.clone(), path.clone(), tx.clone());
                true
            }
            None => false,
        };
        #[cfg(target_arch = "wasm32")]
        let bf_loading = false;

        Self {
            open_window: GuiWindow::default(),
            tx,
            rx,
            bigfile: None,
            bigfile_path: None,
            bigfile_loading: bf_loading,
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
                #[cfg(not(target_arch = "wasm32"))]
                ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                    "{} - {}",
                    TITLE,
                    path.to_string_lossy()
                )));
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

        // egui::CentralPanel::default()
        //     .frame(egui::Frame::none().inner_margin(egui::Margin::same(0.0)))
        // .show(ctx, |ui| {
        let menubar_response = self.menubar_panel(ctx, frame, "menubar".into());
        if menubar_response {
            self.bigfile_loading = true;
        }

        self.bottom_panel(ctx, "bottom".into());

        let resource_list_response = self.resource_list_panel(
            ctx,
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

        self.resource_info_panel(ctx);
        self.preview_panel(ctx);

        match self.open_window {
            GuiWindow::Rename => {
                let mut is_open = true;
                egui::Window::new("Change resource nickname")
                    .open(&mut is_open)
                    .fixed_size(egui::vec2(100.0, 50.0))
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            let output = egui::TextEdit::singleline(&mut self.nickname_editing.1)
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
        // });

        #[cfg(not(target_arch = "wasm32"))]
        {
            preview_files_being_dropped(ctx);

            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    let path = i.raw.dropped_files.first().unwrap().path.as_ref().unwrap();
                    load_bf(ctx.clone(), path.clone(), self.tx.clone());
                }
            });
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
