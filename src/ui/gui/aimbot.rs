use egui::{DragValue, Ui};
use strum::IntoEnumIterator;

use crate::{
    cs2::{bones::Bones, entity::weapon::Weapon},
    ui::{app::App, gui::helpers::keybind},
};

#[derive(Clone, PartialEq)]
pub enum AimbotTab {
    Global,
    Weapon,
}

fn weapon_selector(ui: &mut Ui, label: &str, weapon: &mut Weapon) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::new(ui.next_auto_id(), "")
            .selected_text(weapon.to_string())
            .show_ui(ui, |ui| {
                for w in Weapon::iter() {
                    if matches!(
                        w,
                        Weapon::Unknown
                            | Weapon::Knife
                            | Weapon::Taser
                            | Weapon::Flashbang
                            | Weapon::HeGrenade
                            | Weapon::Smoke
                            | Weapon::Molotov
                            | Weapon::Decoy
                            | Weapon::Incendiary
                            | Weapon::C4
                    ) {
                        continue;
                    }
                    let name = w.to_string();
                    if ui.selectable_label(*weapon == w, &name).clicked() {
                        *weapon = w;
                        changed = true;
                    }
                }
            });
    });
    changed
}

fn config_scope_selector(ui: &mut Ui, current_tab: &mut AimbotTab) -> bool {
    let mut changed = false;
    let tabs = ["Global", "Per-Weapon"];
    let tab_idx = if *current_tab == AimbotTab::Global { 0 } else { 1 };
    ui.horizontal(|ui| {
        ui.label("Scope:");
        egui::ComboBox::new("config_scope", "")
            .selected_text(tabs[tab_idx])
            .show_ui(ui, |ui| {
                for (i, t) in tabs.iter().enumerate() {
                    if ui.selectable_label(tab_idx == i, *t).clicked() {
                        *current_tab = if i == 0 { AimbotTab::Global } else { AimbotTab::Weapon };
                        changed = true;
                    }
                }
            });
    });
    changed
}

impl App {
    pub fn aimbot_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= config_scope_selector(ui, &mut self.aimbot_tab);

        if self.aimbot_tab == AimbotTab::Weapon {
            changed |= weapon_selector(ui, "Weapon:", &mut self.aimbot_weapon);
        }

        if keybind(ui, "aim_key", "Hotkey", &mut self.config.aim.aimbot_hotkey) {
            changed = true;
        }

        ui.add_space(4.0);

        let tab = self.aimbot_tab.clone();
        let weapon = self.aimbot_weapon.clone();
        let cfg = self.weapon_config_for(&tab, &weapon);

        if tab == AimbotTab::Weapon {
            changed |= ui
                .checkbox(&mut cfg.aimbot.enable_override, "Enable Override")
                .changed();
        }

        changed |= ui
            .checkbox(&mut cfg.aimbot.enabled, "Enable")
            .changed();

        ui.add_space(4.0);

        {
            let modes = ["Hold", "Toggle"];
            let mode_idx = if cfg.aimbot.mode == crate::config::KeyMode::Hold { 0 } else { 1 };
            ui.horizontal(|ui| {
                ui.label("Mode");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::new("aim_mode", "")
                        .selected_text(modes[mode_idx])
                        .show_ui(ui, |ui| {
                            for (i, m) in modes.iter().enumerate() {
                                if ui.selectable_label(mode_idx == i, *m).clicked() {
                                    cfg.aimbot.mode = if i == 0 { crate::config::KeyMode::Hold } else { crate::config::KeyMode::Toggle };
                                    changed = true;
                                }
                            }
                        });
                });
            });
        }

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("FOV");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut cfg.aimbot.fov).range(0.1..=360.0).speed(0.02).max_decimals(1).suffix("°"))
                }).response.changed()
            })
            .inner;

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Smooth");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut cfg.aimbot.smooth).range(0.0..=20.0).speed(0.02).max_decimals(1))
                }).response.changed()
            })
            .inner;

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Start Bullet");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut cfg.aimbot.start_bullet).range(0..=10).speed(0.05))
                }).response.changed()
            })
            .inner;

        ui.add_space(4.0);

        changed |= ui
            .checkbox(&mut cfg.aimbot.target_friendlies, "Target Friendlies")
            .changed();
        changed |= ui
            .checkbox(&mut cfg.aimbot.distance_adjusted_fov, "Distance-Adjusted FOV")
            .changed();
        changed |= ui
            .checkbox(&mut cfg.aimbot.visibility_check, "Visibility Check")
            .changed();
        changed |= ui
            .checkbox(&mut cfg.aimbot.flash_check, "Flash Check")
            .changed();

        ui.add_space(4.0);

        {
            let modes = ["FOV", "Distance"];
            let mode_idx = if cfg.aimbot.targeting_mode == crate::config::TargetingMode::Fov { 0 } else { 1 };
            ui.horizontal(|ui| {
                ui.label("Targeting Mode");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::new("aim_targeting", "")
                        .selected_text(modes[mode_idx])
                        .show_ui(ui, |ui| {
                            for (i, m) in modes.iter().enumerate() {
                                if ui.selectable_label(mode_idx == i, *m).clicked() {
                                    cfg.aimbot.targeting_mode = if i == 0 { crate::config::TargetingMode::Fov } else { crate::config::TargetingMode::Distance };
                                    changed = true;
                                }
                            }
                        });
                });
            });
        }

        ui.add_space(4.0);
        {
            let count = cfg.aimbot.bones.len();
            let total = Bones::iter().count();
            ui.horizontal(|ui| {
                ui.label(format!("Bones: {}/{}", count, total));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Select").clicked() {
                        self.show_bone_selector = true;
                    }
                });
            });
        }

        if changed {
            self.send_config();
        }
    }

    pub fn triggerbot_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= config_scope_selector(ui, &mut self.triggerbot_tab);

        if self.triggerbot_tab == AimbotTab::Weapon {
            changed |= weapon_selector(ui, "Weapon:", &mut self.triggerbot_weapon);
        }

        if keybind(ui, "trig_key", "Hotkey", &mut self.config.aim.triggerbot_hotkey) {
            changed = true;
        }

        ui.add_space(4.0);

        let tab = self.triggerbot_tab.clone();
        let weapon = self.triggerbot_weapon.clone();
        let cfg = self.weapon_config_for(&tab, &weapon);

        if tab == AimbotTab::Weapon {
            changed |= ui
                .checkbox(&mut cfg.triggerbot.enable_override, "Enable Override")
                .changed();
        }

        changed |= ui
            .checkbox(&mut cfg.triggerbot.enabled, "Enable")
            .changed();

        ui.add_space(4.0);

        {
            let mut start = *cfg.triggerbot.delay.start();
            let mut end = *cfg.triggerbot.delay.end();

            ui.horizontal(|ui| {
                ui.label("Delay");
                changed |= ui.add(DragValue::new(&mut start).range(0..=999).speed(1.0)).changed();
                changed |= ui.add(DragValue::new(&mut end).range(0..=999).speed(1.0)).changed();
            });

            if start > end {
                std::mem::swap(&mut start, &mut end);
            }
            cfg.triggerbot.delay = start..=end;
        }

        ui.add_space(4.0);

        {
            let modes = ["Hold", "Toggle"];
            let mode_idx = if cfg.triggerbot.mode == crate::config::KeyMode::Hold { 0 } else { 1 };
            ui.horizontal(|ui| {
                ui.label("Mode");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::new("trig_mode", "")
                        .selected_text(modes[mode_idx])
                        .show_ui(ui, |ui| {
                            for (i, m) in modes.iter().enumerate() {
                                if ui.selectable_label(mode_idx == i, *m).clicked() {
                                    cfg.triggerbot.mode = if i == 0 { crate::config::KeyMode::Hold } else { crate::config::KeyMode::Toggle };
                                    changed = true;
                                }
                            }
                        });
                });
            });
        }

        ui.add_space(4.0);

        changed |= ui
            .checkbox(&mut cfg.triggerbot.head_only, "Head Only")
            .changed();
        changed |= ui
            .checkbox(&mut cfg.triggerbot.flash_check, "Flash Check")
            .changed();
        changed |= ui
            .checkbox(&mut cfg.triggerbot.scope_check, "Scope Check")
            .changed();
        changed |= ui
            .checkbox(&mut cfg.triggerbot.velocity_check, "Velocity Check")
            .changed();

        if changed {
            self.send_config();
        }
    }

    pub fn rcs_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= config_scope_selector(ui, &mut self.rcs_tab);

        if self.rcs_tab == AimbotTab::Weapon {
            changed |= weapon_selector(ui, "Weapon:", &mut self.rcs_weapon);
        }

        ui.add_space(4.0);

        let tab = self.rcs_tab.clone();
        let weapon = self.rcs_weapon.clone();
        let cfg = self.weapon_config_for(&tab, &weapon);

        if tab == AimbotTab::Weapon {
            changed |= ui
                .checkbox(&mut cfg.rcs.enable_override, "Enable Override")
                .changed();
        }

        changed |= ui
            .checkbox(&mut cfg.rcs.enabled, "Enable")
            .changed();

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("X Strength");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut cfg.rcs.strength.x).range(0.0..=1.0).speed(0.01))
                }).response.changed()
            })
            .inner;

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Y Strength");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut cfg.rcs.strength.y).range(0.0..=1.0).speed(0.01))
                }).response.changed()
            })
            .inner;

        if changed {
            self.send_config();
        }
    }
}
