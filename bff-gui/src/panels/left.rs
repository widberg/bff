use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::Resource;
use bff::bigfile::versions::Version;
use bff::class::Class;
use bff::names::{Name, NameContext};
use bff::traits::TryIntoVersionPlatform;
use regex::Regex;

use crate::Gui;
use crate::artifact::Artifact;
use crate::helpers::artifact::{class_supports_preview, create_artifact};

#[derive(Clone, PartialEq, Default)]
enum SortType {
    #[default]
    Name,
    Ext,
    Size,
}

#[derive(Default, Clone, PartialEq)]
struct ListSort {
    sort_type: SortType,
    reverse: bool,
}

#[derive(Default, Clone, PartialEq)]
struct ResourceListState {
    sort: ListSort,
    filter: Option<HashMap<Name, bool>>,
    name_filter: String,
    name_filter_regex: bool,
    resources: Option<Arc<Vec<Name>>>,
    filter_order: Option<Vec<Name>>,
}

#[derive(Default)]
pub struct ResourceListResponse {
    pub resource_context_menu: Option<Name>,
    pub resource_clicked: Option<Name>,
    pub artifact_created: Option<Artifact>,
    pub info_created: Option<String>,
    pub nickname_cleared: Option<Name>,
}

fn load_artifact(
    artifacts: &HashMap<Name, Artifact>,
    infos: &HashMap<Name, String>,
    bigfile: &BigFile,
    resource: &Name,
    version: &Version,
    platform: Platform,
    name_context: &NameContext,
) -> (Option<String>, Option<Artifact>) {
    if artifacts.get(resource).is_none() || infos.get(resource).is_none() {
        match bigfile
            .resources
            .get(resource)
            .unwrap()
            .try_into_version_platform(version.clone(), platform)
        {
            Ok(class) => {
                return (
                    Some(
                        bff::names::json::to_string_pretty::<Class>(&class, name_context).unwrap(),
                    ),
                    create_artifact(bigfile, class),
                );
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    (None, None)
}

fn resource_display_label(
    resource: &Resource,
    nicknames: &HashMap<Name, String>,
    name_context: &NameContext,
) -> String {
    let display_name = nicknames
        .get(&resource.name)
        .cloned()
        .unwrap_or_else(|| resource.name.with_context(name_context).to_string());
    let class_name = resource.class_name.with_context(name_context).to_string();
    format!("{display_name}.{class_name}")
}

impl Gui {
    pub fn resource_list_panel(
        &mut self,
        ctx: &egui::Context,
        id_source: egui::Id,
    ) -> ResourceListResponse {
        let mut response = ResourceListResponse::default();
        let mut changed_list = false;
        egui::SidePanel::left("left")
            .resizable(true)
            .show(ctx, |ui: &mut egui::Ui| {
                if let Some(bigfile) = &self.bigfile {
                    let version = &bigfile.manifest.version;
                    let platform = bigfile.manifest.platform;
                    let binding = match ui.memory(|mem| {
                        mem.data
                            .get_temp::<Arc<Mutex<ResourceListState>>>(id_source)
                    }) {
                        Some(val) => val,
                        None => Arc::new(Mutex::new(ResourceListState::default())),
                    };
                    let state = binding.lock().unwrap();
                    let mut new_state = state.clone();
                    let mut class_names = new_state.filter.unwrap_or(
                        bigfile
                            .resources
                            .values()
                            .map(|res| res.class_name)
                            .collect::<HashSet<_>>()
                            .iter()
                            .map(|n| (*n, true))
                            .collect(),
                    );
                    let sorted_filter = new_state.filter_order.get_or_insert_with(|| {
                        let mut filters: Vec<Name> = class_names.keys().cloned().collect();
                        filters.sort_by_cached_key(|k| {
                            k.with_context(self.name_context.as_ref()).to_string()
                        });
                        filters
                    });
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 1.0;
                            if ui
                                .add(
                                    egui::Button::new(match new_state.sort.sort_type {
                                        SortType::Name => "Name",
                                        SortType::Ext => "Extension",
                                        SortType::Size => "Size",
                                    })
                                    .min_size(egui::vec2(ui.available_width() / 3.0, 0.0)),
                                )
                                .clicked()
                            {
                                changed_list = true;
                                new_state.sort.sort_type = match new_state.sort.sort_type {
                                    SortType::Name => SortType::Ext,
                                    SortType::Ext => SortType::Size,
                                    SortType::Size => SortType::Name,
                                };
                            }
                            if ui
                                .button(
                                    egui::RichText::new(match new_state.sort.reverse {
                                        false => "",
                                        true => "",
                                    })
                                    .family(egui::FontFamily::Name("icons".into())),
                                )
                                .clicked()
                            {
                                changed_list = true;
                                new_state.sort.reverse = !new_state.sort.reverse;
                            }
                        });

                        ui.style_mut().spacing.item_spacing.x = 5.0;
                        ui.horizontal(|ui| {
                            ui.menu_button("Filter", |ui| {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    let mut all_selected =
                                        class_names.iter().all(|(_, checked)| *checked);
                                    if ui.checkbox(&mut all_selected, "Select all").clicked() {
                                        class_names
                                            .iter_mut()
                                            .for_each(|(_, checked)| *checked = all_selected);
                                        changed_list = true;
                                    }
                                    sorted_filter.iter().for_each(|name| {
                                        let checked = class_names.get_mut(name).unwrap();
                                        if ui
                                            .checkbox(
                                                checked,
                                                name.with_context(self.name_context.as_ref())
                                                    .to_string(),
                                            )
                                            .clicked()
                                        {
                                            changed_list = true;
                                        }
                                    });
                                });
                            });
                            let name_filter_response = ui.add(
                                egui::TextEdit::singleline(&mut new_state.name_filter)
                                    .hint_text("Filter by name")
                                    .desired_width(175.0),
                            );
                            if name_filter_response.changed() {
                                changed_list = true;
                            }
                            if ui
                                .checkbox(&mut new_state.name_filter_regex, "Regex")
                                .changed()
                            {
                                changed_list = true;
                            }
                        });
                    });
                    new_state.filter = Some(class_names);
                    let name_filter = new_state.name_filter.trim().to_owned();
                    let name_filter_lower = name_filter.to_ascii_lowercase();
                    let mut regex_error = None;
                    let name_filter_regex =
                        if new_state.name_filter_regex && !name_filter.is_empty() {
                            match Regex::new(&name_filter) {
                                Ok(regex) => Some(regex),
                                Err(error) => {
                                    regex_error = Some(error.to_string());
                                    None
                                }
                            }
                        } else {
                            None
                        };
                    let class_filter = new_state.filter.as_ref();
                    let nicknames = &self.nicknames;
                    let name_context = self.name_context.as_ref();
                    let is_resource_visible = |res: &Resource| {
                        if !class_filter
                            .and_then(|filters| filters.get(&res.class_name))
                            .copied()
                            .unwrap_or(true)
                        {
                            return false;
                        }
                        if name_filter.is_empty() {
                            return true;
                        }
                        let displayed_resource = resource_display_label(res, nicknames, name_context);
                        if let Some(regex) = &name_filter_regex {
                            regex.is_match(displayed_resource.as_str())
                        } else {
                            displayed_resource
                                .to_ascii_lowercase()
                                .contains(&name_filter_lower)
                        }
                    };
                    ui.horizontal(|ui| {
                        if let Some(error) = regex_error {
                            ui.colored_label(
                                egui::Color32::from_rgb(245, 111, 111),
                                format!("Invalid regex: {}", error),
                            );
                        }
                        let filtered_count = bigfile
                            .resources
                            .values()
                            .filter(|res| is_resource_visible(*res))
                            .count();
                        ui.label(format!("{}/{}", filtered_count, bigfile.resources.len()));
                    });

                    let resources: Arc<Vec<Name>> = match (&new_state.resources, changed_list) {
                        (None, _) | (_, true) => {
                            let mut res: Vec<(Name, Name, usize)> = bigfile
                                .resources
                                .values()
                                .filter(|res| is_resource_visible(*res))
                                .map(|r| (r.name, r.class_name, r.size()))
                                .collect();
                            match new_state.sort.sort_type {
                                SortType::Name => res.sort_by_cached_key(|k| {
                                    k.0.with_context(self.name_context.as_ref()).to_string()
                                }),
                                SortType::Ext => res.sort_by_cached_key(|k| {
                                    k.1.with_context(self.name_context.as_ref()).to_string()
                                }),
                                SortType::Size => res.sort_by_cached_key(|k| k.2),
                            }
                            if new_state.sort.reverse {
                                res.reverse();
                            }
                            let only_names: Arc<Vec<Name>> =
                                Arc::new(res.into_iter().map(|(name, _, _)| name).collect());
                            new_state.resources = Some(Arc::clone(&only_names));
                            Arc::clone(&only_names)
                        }
                        (Some(resources), _) => Arc::clone(resources),
                    };
                    if new_state != *state {
                        ui.memory_mut(|mem| {
                            mem.data
                                .insert_temp(id_source, Arc::new(Mutex::new(new_state.clone())))
                        });
                    }

                    ui.add(egui::Separator::default().horizontal().spacing(1.0));

                    ui.style_mut().spacing.interact_size.y += 2.0;
                    let row_height = ui.spacing().interact_size.y;
                    egui::ScrollArea::vertical().show_rows(
                        ui,
                        row_height,
                        resources.len(),
                        |ui, row_range| {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::LEFT),
                                |ui| {
                                    ui.set_min_width(10.0);
                                    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                        if let Some(cur) = self.resource_name {
                                            let mut res_iter = resources.iter().cycle();
                                            if res_iter.any(|n| n == &cur) {
                                                let res = *res_iter.next().unwrap();
                                                response.resource_clicked = Some(res);
                                                (
                                                    response.info_created,
                                                    response.artifact_created,
                                                ) = load_artifact(
                                                    &self.artifacts,
                                                    &self.infos,
                                                    bigfile,
                                                    &res,
                                                    version,
                                                    platform,
                                                    self.name_context.as_ref(),
                                                );
                                            }
                                        }
                                    }
                                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                        if let Some(cur) = self.resource_name {
                                            let mut res_iter = resources.iter();
                                            if let Some(i) = res_iter.position(|n| n == &cur) {
                                                let c = if i == 0 {
                                                    resources.len() - 1
                                                } else {
                                                    i - 1
                                                };
                                                let res = *resources.get(c).unwrap();
                                                response.resource_clicked = Some(res);
                                                (
                                                    response.info_created,
                                                    response.artifact_created,
                                                ) = load_artifact(
                                                    &self.artifacts,
                                                    &self.infos,
                                                    bigfile,
                                                    &res,
                                                    version,
                                                    platform,
                                                    self.name_context.as_ref(),
                                                );
                                            }
                                        }
                                    }

                                    // RESOURCE BUTTONS
                                    ui.style_mut().spacing.item_spacing.y = 4.0;
                                    for row in row_range {
                                        let resource = resources.get(row).unwrap();
                                        let resource_entry = bigfile.resources.get(resource).unwrap();
                                        let nickname = self.nicknames.get(resource);
                                        let mut tooltip_text = format!(
                                            "Size: {} bytes",
                                            resource_entry.size()
                                        );
                                        if nickname.is_some() {
                                            tooltip_text.push_str(
                                                format!(
                                                    "\nOriginal name: {}",
                                                    resource.with_context(self.name_context.as_ref())
                                                )
                                                .as_str(),
                                            );
                                        }
                                        let button_label = resource_display_label(
                                            resource_entry,
                                            &self.nicknames,
                                            self.name_context.as_ref(),
                                        );
                                        let button_text = if class_supports_preview(
                                            resource_entry.class_name,
                                            version,
                                            platform,
                                        ) {
                                            egui::RichText::new(button_label)
                                                .color(egui::Color32::from_rgb(120, 220, 140))
                                        } else {
                                            egui::RichText::new(button_label)
                                        };
                                        let btn = ui
                                            .add(
                                                egui::Button::new(button_text)
                                                .rounding(0.0)
                                                .truncate()
                                                .selected(
                                                    self.resource_name
                                                        .map_or_else(|| false, |n| resource == &n),
                                                ),
                                            )
                                            .on_hover_text_at_pointer(tooltip_text);
                                        btn.context_menu(|ui| {
                                            if ui.button("Change nickname").clicked() {
                                                ui.close_menu();
                                                response.resource_context_menu = Some(*resource);
                                            }
                                            if ui
                                                .add_enabled(
                                                    nickname.is_some(),
                                                    egui::Button::new("Clear nickname"),
                                                )
                                                .clicked()
                                            {
                                                ui.close_menu();
                                                response.nickname_cleared = Some(*resource);
                                            }
                                        });
                                        if btn.clicked() {
                                            response.resource_clicked = Some(*resource);
                                            (response.info_created, response.artifact_created) =
                                                load_artifact(
                                                    &self.artifacts,
                                                    &self.infos,
                                                    bigfile,
                                                    resource,
                                                    version,
                                                    platform,
                                                    self.name_context.as_ref(),
                                                );
                                        }
                                    }
                                },
                            );
                        },
                    );
                }
            });
        response
    }
}
