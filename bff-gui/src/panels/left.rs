use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;

use crate::artifact::Artifact;
use crate::helpers::artifact::create_artifact;
use crate::Gui;

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

impl Gui {
    pub fn resource_list_panel(
        &mut self,
        ui: &mut egui::Ui,
        id_source: egui::Id,
    ) -> ResourceListResponse {
        let mut response = ResourceListResponse::default();
        let mut changed_list = false;
        egui::SidePanel::left("left")
            .resizable(true)
            .width_range(70.0..=ui.available_width() / 2.0)
            .show_inside(ui, |ui| {
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
                            .objects
                            .values()
                            .map(|res| res.class_name)
                            .collect::<HashSet<_>>()
                            .iter()
                            .map(|n| (*n, true))
                            .collect(),
                    );
                    let sorted_filter = new_state.filter_order.get_or_insert_with(|| {
                        let mut filters: Vec<Name> = class_names.keys().cloned().collect();
                        filters.sort_by_cached_key(|k| k.to_string());
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
                                        if ui.checkbox(checked, name.to_string()).clicked() {
                                            changed_list = true;
                                        }
                                    });
                                });
                            });
                            ui.label(format!(
                                "{}/{}",
                                bigfile
                                    .objects
                                    .values()
                                    .filter(|res| {
                                        *class_names.get(&res.class_name).unwrap_or(&true)
                                    })
                                    .collect::<Vec<_>>()
                                    .len(),
                                bigfile.objects.len()
                            ));
                        });
                    });
                    new_state.filter = Some(class_names);

                    let resources: Arc<Vec<Name>> = if new_state.resources.is_none() || changed_list
                    {
                        let mut res: Vec<(Name, Name, usize)> = bigfile
                            .objects
                            .values()
                            .filter(|res| {
                                *new_state
                                    .filter
                                    .as_ref()
                                    .unwrap_or(&HashMap::default())
                                    .get(&res.class_name)
                                    .unwrap_or(&true)
                            })
                            .map(|r| (r.name, r.class_name, r.size()))
                            .collect();
                        match new_state.sort.sort_type {
                            SortType::Name => res.sort_by_cached_key(|k| k.0.to_string()),
                            SortType::Ext => res.sort_by_cached_key(|k| k.1.to_string()),
                            SortType::Size => res.sort_by_cached_key(|k| k.2),
                        }
                        if new_state.sort.reverse {
                            res.reverse();
                        }
                        let only_names: Arc<Vec<Name>> =
                            Arc::new(res.into_iter().map(|(name, _, _)| name).collect());
                        new_state.resources = Some(Arc::clone(&only_names));
                        Arc::clone(&only_names)
                    } else {
                        Arc::clone(new_state.resources.as_ref().unwrap())
                    };
                    if new_state != *state {
                        ui.memory_mut(|mem| {
                            mem.data
                                .insert_temp(id_source, Arc::new(Mutex::new(new_state.clone())))
                        });
                    }

                    ui.add(egui::Separator::default().horizontal().spacing(1.0));

                    let row_height = ui.spacing().interact_size.y;
                    egui::ScrollArea::vertical().show_rows(
                        ui,
                        row_height,
                        resources.len(),
                        |ui, row_range| {
                            ui.set_min_width(ui.available_width());
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                if let Some(cur) = self.resource_name {
                                    let mut res_iter = resources.iter().cycle();
                                    if res_iter.any(|n| n == &cur) {
                                        let res = *res_iter.next().unwrap();
                                        response.resource_clicked = Some(res);
                                        if self.artifacts.get(&res).is_none()
                                            || self.infos.get(&res).is_none()
                                        {
                                            match bigfile
                                                .objects
                                                .get(&res)
                                                .unwrap()
                                                .try_into_version_platform(
                                                    version.clone(),
                                                    platform,
                                                ) {
                                                Ok(class) => {
                                                    response.info_created = Some(
                                                        serde_json::to_string_pretty::<Class>(
                                                            &class,
                                                        )
                                                        .unwrap(),
                                                    );
                                                    response.artifact_created =
                                                        create_artifact(bigfile, class);
                                                }
                                                Err(e) => {
                                                    println!("{:?}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                if let Some(cur) = self.resource_name {
                                    let mut res_iter = resources.iter();
                                    if let Some(i) = res_iter.position(|n| n == &cur) {
                                        let c = if i == 0 { resources.len() - 1 } else { i - 1 };
                                        let res = *resources.get(c).unwrap();
                                        response.resource_clicked = Some(res);
                                        if self.artifacts.get(&res).is_none()
                                            || self.infos.get(&res).is_none()
                                        {
                                            match bigfile
                                                .objects
                                                .get(&res)
                                                .unwrap()
                                                .try_into_version_platform(
                                                    version.clone(),
                                                    platform,
                                                ) {
                                                Ok(class) => {
                                                    response.info_created = Some(
                                                        serde_json::to_string_pretty::<Class>(
                                                            &class,
                                                        )
                                                        .unwrap(),
                                                    );
                                                    response.artifact_created =
                                                        create_artifact(bigfile, class);
                                                }
                                                Err(e) => {
                                                    println!("{:?}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // ui.style_mut().spacing.item_spacing.y = 4.0;

                            for row in row_range {
                                let resource = resources.get(row).unwrap();
                                let nickname = self.nicknames.get(resource);
                                let mut tooltip_text = format!(
                                    "Size: {} bytes",
                                    bigfile.objects.get(resource).unwrap().size()
                                );
                                if nickname.is_some() {
                                    tooltip_text.push_str(
                                        format!("\nOriginal name: {}", resource).as_str(),
                                    );
                                }
                                let btn = ui
                                    .add(
                                        egui::Button::new(format!(
                                            "{}.{}",
                                            match nickname {
                                                Some(nn) => nn.to_owned(),
                                                None => resource.to_string(),
                                            },
                                            bigfile.objects.get(resource).unwrap().class_name
                                        ))
                                        .rounding(0.0)
                                        .min_size(egui::vec2(ui.available_width(), 0.0))
                                        .wrap(true)
                                        .selected(
                                            if let Some(n) = &self.resource_name {
                                                resource == n
                                            } else {
                                                false
                                            },
                                        ),
                                    )
                                    .context_menu(|ui| {
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
                                    })
                                    .on_hover_text_at_pointer(tooltip_text);
                                if btn.clicked() {
                                    response.resource_clicked = Some(*resource);
                                    if self.artifacts.get(resource).is_none()
                                        || self.infos.get(resource).is_none()
                                    {
                                        match bigfile
                                            .objects
                                            .get(resource)
                                            .unwrap()
                                            .try_into_version_platform(version.clone(), platform)
                                        {
                                            Ok(class) => {
                                                response.info_created = Some(
                                                    serde_json::to_string_pretty::<Class>(&class)
                                                        .unwrap(),
                                                );
                                                response.artifact_created =
                                                    create_artifact(bigfile, class);
                                            }
                                            Err(e) => {
                                                println!("{:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    );
                }
            });
        response
    }
}
