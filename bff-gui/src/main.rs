#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use artifact::Artifact;
use bff::bigfile::BigFile;
use bff::names::{Name, NameContext};
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
use mimalloc::MiMalloc;

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
mod registry {
    use std::io::{Error, ErrorKind, Result};

    use windows::Win32::Foundation::{
        ERROR_FILE_NOT_FOUND,
        ERROR_PATH_NOT_FOUND,
        ERROR_SUCCESS,
        WIN32_ERROR,
    };
    use windows::Win32::System::Registry::{
        HKEY,
        KEY_READ,
        KEY_WRITE,
        REG_OPTION_NON_VOLATILE,
        REG_ROUTINE_FLAGS,
        REG_SAM_FLAGS,
        REG_SZ,
        RRF_RT_REG_SZ,
        RegCloseKey,
        RegCreateKeyExW,
        RegDeleteTreeW,
        RegDeleteValueW,
        RegGetValueW,
        RegOpenKeyExW,
        RegSetValueExW,
    };
    use windows::core::PCWSTR;

    pub struct Key(HKEY);

    impl Key {
        pub fn raw(&self) -> HKEY {
            self.0
        }
    }

    impl Drop for Key {
        fn drop(&mut self) {
            unsafe {
                let _ = RegCloseKey(self.0);
            }
        }
    }

    fn is_not_found(status: WIN32_ERROR) -> bool {
        status == ERROR_FILE_NOT_FOUND || status == ERROR_PATH_NOT_FOUND
    }

    fn win32_error(status: WIN32_ERROR) -> Error {
        Error::from_raw_os_error(status.0 as i32)
    }

    fn status_to_result(status: WIN32_ERROR) -> Result<()> {
        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(win32_error(status))
        }
    }

    fn widestr(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn open_subkey(parent: HKEY, subkey: &str, access: REG_SAM_FLAGS) -> Result<Key> {
        let mut handle = HKEY::default();
        let subkey = widestr(subkey);
        unsafe {
            status_to_result(RegOpenKeyExW(
                parent,
                PCWSTR(subkey.as_ptr()),
                None,
                access,
                &mut handle,
            ))?;
        }
        Ok(Key(handle))
    }

    pub fn create_subkey(parent: HKEY, subkey: &str) -> Result<Key> {
        let mut handle = HKEY::default();
        let subkey = widestr(subkey);
        unsafe {
            status_to_result(RegCreateKeyExW(
                parent,
                PCWSTR(subkey.as_ptr()),
                None,
                PCWSTR::null(),
                REG_OPTION_NON_VOLATILE,
                KEY_READ | KEY_WRITE,
                None,
                &mut handle,
                None,
            ))?;
        }
        Ok(Key(handle))
    }

    pub fn set_default_value_sz(key: HKEY, value: &str) -> Result<()> {
        set_value_sz(key, None, value)
    }

    pub fn set_value_sz(key: HKEY, name: Option<&str>, value: &str) -> Result<()> {
        let value = widestr(value);
        let value_bytes = unsafe {
            std::slice::from_raw_parts(
                value.as_ptr() as *const u8,
                std::mem::size_of_val(value.as_slice()),
            )
        };
        unsafe {
            match name {
                Some(name) => {
                    let name = widestr(name);
                    status_to_result(RegSetValueExW(
                        key,
                        PCWSTR(name.as_ptr()),
                        None,
                        REG_SZ,
                        Some(value_bytes),
                    ))
                }
                None => status_to_result(RegSetValueExW(
                    key,
                    PCWSTR::null(),
                    None,
                    REG_SZ,
                    Some(value_bytes),
                )),
            }
        }
    }

    pub fn get_default_value_sz(key: HKEY) -> Result<Option<String>> {
        let mut data_len = 0u32;
        let status = unsafe {
            RegGetValueW(
                key,
                PCWSTR::null(),
                PCWSTR::null(),
                REG_ROUTINE_FLAGS(RRF_RT_REG_SZ.0),
                None,
                None,
                Some(&mut data_len),
            )
        };
        if is_not_found(status) {
            return Ok(None);
        }
        status_to_result(status)?;

        if data_len == 0 {
            return Ok(Some(String::new()));
        }

        let mut data = vec![0u8; data_len as usize];
        let status = unsafe {
            RegGetValueW(
                key,
                PCWSTR::null(),
                PCWSTR::null(),
                REG_ROUTINE_FLAGS(RRF_RT_REG_SZ.0),
                None,
                Some(data.as_mut_ptr() as *mut core::ffi::c_void),
                Some(&mut data_len),
            )
        };
        status_to_result(status)?;

        let wide_len = (data_len as usize) / std::mem::size_of::<u16>();
        let wide = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u16, wide_len) };
        let value_len = wide.iter().position(|c| *c == 0).unwrap_or(wide.len());
        String::from_utf16(&wide[..value_len])
            .map(Some)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))
    }

    pub fn delete_tree_if_exists(parent: HKEY, subkey: &str) -> Result<()> {
        let subkey = widestr(subkey);
        let status = unsafe { RegDeleteTreeW(parent, PCWSTR(subkey.as_ptr())) };
        if is_not_found(status) {
            Ok(())
        } else {
            status_to_result(status)
        }
    }

    pub fn delete_default_value_if_exists(key: HKEY) -> Result<()> {
        delete_value_if_exists(key, None)
    }

    pub fn delete_value_if_exists(key: HKEY, name: Option<&str>) -> Result<()> {
        let status = unsafe {
            match name {
                Some(name) => {
                    let name = widestr(name);
                    RegDeleteValueW(key, PCWSTR(name.as_ptr()))
                }
                None => RegDeleteValueW(key, PCWSTR::null()),
            }
        };
        if is_not_found(status) {
            Ok(())
        } else {
            status_to_result(status)
        }
    }
}

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

    use windows::Win32::System::Registry::HKEY_CURRENT_USER;

    let exe_path = current_exe()?.to_str().unwrap_or_default().to_owned();

    let classes = registry::create_subkey(HKEY_CURRENT_USER, "Software\\Classes")?;
    let prog = registry::create_subkey(classes.raw(), PROG_ID)?;
    registry::set_default_value_sz(prog.raw(), TITLE)?;
    let command = registry::create_subkey(prog.raw(), "Shell\\Open\\Command")?;
    registry::set_default_value_sz(command.raw(), &format!(r#""{}" "%1""#, exe_path))?;

    for extension in bff::bigfile::platforms::extensions() {
        let extension_key =
            registry::create_subkey(classes.raw(), &format!(".{}", extension.to_string_lossy()))?;
        registry::set_default_value_sz(extension_key.raw(), PROG_ID)?;
        let open_with = registry::create_subkey(extension_key.raw(), "OpenWithProgids")?;
        registry::set_value_sz(open_with.raw(), Some(PROG_ID), "")?;
    }

    change_notify();

    Ok(())
}

#[cfg(target_os = "windows")]
fn uninstall() -> BffGuiResult<()> {
    use windows::Win32::System::Registry::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};

    let classes =
        registry::open_subkey(HKEY_CURRENT_USER, "Software\\Classes", KEY_READ | KEY_WRITE)?;
    registry::delete_tree_if_exists(classes.raw(), PROG_ID)?;

    for extension in bff::bigfile::platforms::extensions() {
        let extension_key_name = format!(".{}", extension.to_string_lossy());
        if let Ok(default) =
            registry::open_subkey(classes.raw(), &extension_key_name, KEY_READ | KEY_WRITE)
        {
            if let Some(prog_id) = registry::get_default_value_sz(default.raw())?
                && prog_id == PROG_ID
            {
                registry::delete_default_value_if_exists(default.raw())?;
            }

            if let Ok(open_with) =
                registry::open_subkey(default.raw(), "OpenWithProgids", KEY_READ | KEY_WRITE)
            {
                registry::delete_value_if_exists(open_with.raw(), Some(PROG_ID))?;
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
    name_context: Arc<NameContext>,
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
        let name_context = Arc::new(NameContext::default());
        #[cfg(not(target_arch = "wasm32"))]
        let bf_loading = match file {
            Some(path) => {
                load_bf(
                    cc.egui_ctx.clone(),
                    path.clone(),
                    tx.clone(),
                    Arc::clone(&name_context),
                );
                true
            }
            None => false,
        };
        #[cfg(target_arch = "wasm32")]
        let bf_loading = false;

        Self {
            open_window: GuiWindow::default(),
            name_context,
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
        let name_context = Arc::clone(&self.name_context);
        name_context.scope(|| {
            if let Ok(res) = self.rx.try_recv() {
                if let Some((bf, path)) = res {
                    crate::helpers::sound::stop_audio_playback();

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
                ctx.request_repaint();
            }

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
                if self.resource_name != Some(name) {
                    crate::helpers::sound::stop_audio_playback();
                }
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

            #[cfg(not(target_arch = "wasm32"))]
            {
                preview_files_being_dropped(ctx);

                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        let path = i.raw.dropped_files.first().unwrap().path.as_ref().unwrap();
                        load_bf(
                            ctx.clone(),
                            path.clone(),
                            self.tx.clone(),
                            Arc::clone(&self.name_context),
                        );
                    }
                });
            }

            if self.bigfile_loading {
                show_bigfile_loading_overlay(ctx);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        crate::helpers::sound::shutdown_audio_on_exit();
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

fn show_bigfile_loading_overlay(ctx: &egui::Context) {
    use egui::*;

    let painter = ctx.layer_painter(LayerId::new(
        Order::Foreground,
        Id::new("bigfile_loading_overlay"),
    ));
    let screen_rect = ctx.screen_rect();
    painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(128));

    Window::new("loading_bigfile_window")
        .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_size(vec2(220.0, 90.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.add(Spinner::new().size(28.0));
                ui.add_space(8.0);
                ui.label("Loading BigFile...");
            });
        });
}
