use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use bff::bigfile::BigFile;
use bff::class::Class;
use bff::names::Name;
use bff::traits::TryIntoVersionPlatform;
use egui;

use crate::{Artifact, Export, RecursiveExport};

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
}

#[derive(Default)]
pub struct ResourceListResponse {
    pub resource_context_menu: Option<Name>,
    pub resource_clicked: Option<Name>,
    pub artifact_created: Option<Artifact>,
    pub info_created: Option<String>,
}

pub fn resource_list(
    ui: &mut egui::Ui,
    id_source: egui::Id,
    bigfile: &Option<BigFile>,
    nicknames: &HashMap<Name, String>,
    artifacts: &HashMap<Name, Artifact>,
    infos: &HashMap<Name, String>,
) -> ResourceListResponse {
    let mut response = ResourceListResponse::default();
    let mut changed_list = false;
    egui::SidePanel::left("left")
        .resizable(true)
        .width_range(70.0..=ui.available_width() / 2.0)
        .show_inside(ui, |ui| {
            // ui.set_width_range(150.0..=200.0);
            if let Some(bigfile) = bigfile {
                ui.style_mut().spacing.item_spacing.y = 5.0;

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
                    ui.style_mut().spacing.item_spacing.x = 5.0;
                    if ui
                        .button(
                            egui::RichText::new(match new_state.sort.reverse {
                                false => "",
                                true => "",
                            })
                            .family(egui::FontFamily::Name("icons".into())),
                        )
                        .clicked()
                    {
                        changed_list = true;
                        new_state.sort.reverse = !new_state.sort.reverse;
                    }
                    ui.menu_button("Filter", |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            class_names.iter_mut().for_each(|(name, checked)| {
                                if ui.checkbox(checked, name.to_string()).clicked() {
                                    changed_list = true;
                                }
                            });
                        });
                    });
                });
                // println!("{:?}", class_names);
                new_state.filter = Some(class_names);

                let resources: Arc<Vec<Name>> = if new_state.resources.is_none() || changed_list {
                    let mut res: Vec<(Name, (Name, usize))> = bigfile
                        .objects
                        .values()
                        .filter(|res| {
                            *state
                                .filter
                                .as_ref()
                                .unwrap_or(&HashMap::default())
                                .get(&res.class_name)
                                .unwrap_or(&true)
                        })
                        .map(|r| (r.name, (r.class_name, r.size())))
                        .collect();
                    // resources.sort_by(compare)
                    res.sort_by_cached_key(|a| match new_state.sort.sort_type {
                        SortType::Name => a.0.to_string(),
                        SortType::Ext => a.1 .0.to_string(),
                        SortType::Size => a.1 .1.to_string(),
                    });
                    if new_state.sort.reverse {
                        res.reverse();
                    }
                    let only_names: Arc<Vec<Name>> =
                        Arc::new(res.into_iter().map(|(name, _)| name).collect());
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

                let row_height = ui.spacing().interact_size.y;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    resources.len(),
                    |ui, row_range| {
                        ui.set_min_width(ui.available_width());
                        ui.style_mut().spacing.item_spacing.y = 4.0;

                        for row in row_range {
                            let resource = resources.get(row).unwrap();
                            let nickname = nicknames.get(resource);
                            let temp_btn = ui
                                .add(
                                    egui::Button::new(format!(
                                        "{}.{}",
                                        match nickname {
                                            Some(nn) => nn.to_owned(),
                                            None => resource.to_string(),
                                        },
                                        bigfile.objects.get(resource).unwrap().class_name
                                    ))
                                    .wrap(false)
                                    .rounding(0.0)
                                    .min_size(egui::vec2(ui.available_width(), 0.0)),
                                )
                                .context_menu(|ui| {
                                    if ui.button("Change nickname").clicked() {
                                        // self.nickname_window_open = true;
                                        // self.nickname_editing.0 = resource.name;
                                        response.resource_context_menu = Some(*resource);
                                        ui.close_menu();
                                    }
                                });
                            let btn = if nickname.is_some() {
                                temp_btn.on_hover_ui_at_pointer(|ui| {
                                    ui.label(resource.to_string());
                                })
                            } else {
                                temp_btn
                            };
                            if btn.clicked() {
                                response.resource_clicked = Some(*resource);
                                if artifacts.get(resource).is_none()
                                    || infos.get(resource).is_none()
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
                                // self.resource_name = Some(resource.name);
                            }
                        }
                    },
                );
            }
        });
    response
}

fn create_artifact(bigfile: &BigFile, class: Class) -> Option<Artifact> {
    match class {
        Class::Bitmap(box_bitmap) => {
            let artifact = match *box_bitmap {
                bff::class::bitmap::Bitmap::BitmapV1_06_63_02PC(bitmap) => bitmap.export(),
                bff::class::bitmap::Bitmap::BitmapV1_291_03_06PC(bitmap) => bitmap.export(),
                bff::class::bitmap::Bitmap::BitmapV1_381_67_09PC(bitmap) => bitmap.export(),
            };
            Some(artifact)
        }
        Class::Sound(box_sound) => {
            let artifact = match *box_sound {
                bff::class::sound::Sound::SoundV1_291_03_06PC(sound) => {
                    // let points = sound.body.data.iter().enumerate().map(|(i, s)| eframe::epaint::Pos2{x: ((i as f32 * ui.available_width()) / sound.body.data.len() as f32), y: (s / 200 + 200).into()}).collect();
                    // let shape = eframe::epaint::PathShape::line(points, eframe::epaint::Stroke::new(1.0, eframe::epaint::Color32::WHITE));
                    // ui.painter().add(shape);
                    sound.export()
                }
                bff::class::sound::Sound::SoundV1_381_67_09PC(sound) => sound.export(),
            };
            Some(artifact)
        }
        Class::Mesh(box_mesh) => match *box_mesh {
            bff::class::mesh::Mesh::MeshV1_291_03_06PC(mesh) => Some(mesh.export()),
            _ => None,
        },
        Class::Skin(box_skin) => match *box_skin {
            bff::class::skin::Skin::SkinV1_291_03_06PC(skin) => {
                // let dependency_names = ;
                let dependency_classes: HashMap<Name, Class> = skin
                    .dependencies()
                    .iter()
                    .map(|n| bigfile.objects.get(n))
                    .flatten()
                    .map(|r| {
                        (
                            r.name,
                            TryIntoVersionPlatform::<Class>::try_into_version_platform(
                                r,
                                bigfile.manifest.version.clone(),
                                bigfile.manifest.platform,
                            )
                            .unwrap(),
                        )
                    })
                    .collect();
                Some(skin.export(&dependency_classes))
            }
            _ => None,
        },
        _ => None,
    }
}