use egui::{DragValue, Ui};

use crate::ui::app::App;

impl App {
    pub fn hud_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui.checkbox(&mut self.config.hud.bomb_timer, "Bomb Timer").changed();
        changed |= ui.checkbox(&mut self.config.hud.fov_circle, "FOV Circle").changed();
        changed |= ui.checkbox(&mut self.config.hud.dropped_weapons, "Dropped Weapons").changed();
        changed |= ui.checkbox(&mut self.config.hud.keybind_list, "Keybind List").changed();
        changed |= ui.checkbox(&mut self.config.hud.spectator_list, "Spectator List").changed();
        changed |= ui.checkbox(&mut self.config.hud.grenade_trails, "Grenade Trails").changed();
        changed |= ui.checkbox(&mut self.config.hud.grenade_predict, "Grenade Predict").changed();
        changed |= ui.checkbox(&mut self.config.hud.text_outline, "Text Outline").changed();

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Line Width");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.hud.line_width).range(0.1..=8.0).speed(0.02))
                }).response.changed()
            })
            .inner;

        changed |= ui
            .horizontal(|ui| {
                ui.label("Font Size");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.hud.font_size).range(1.0..=99.0).speed(0.2))
                }).response.changed()
            })
            .inner;

        changed |= ui
            .horizontal(|ui| {
                ui.label("Icon Size");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.hud.icon_size).range(1.0..=99.0).speed(0.2))
                }).response.changed()
            })
            .inner;

        if changed {
            self.send_config();
        }
    }

    pub fn bomb_timer_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;
        changed |= ui.checkbox(&mut self.config.hud.bomb_timer, "Bomb Timer").changed();
        ui.label("Shows the bomb timer on screen.");
        if changed {
            self.send_config();
        }
    }

    pub fn fov_circle_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;
        changed |= ui.checkbox(&mut self.config.hud.fov_circle, "FOV Circle").changed();
        ui.label("Shows the aimbot FOV circle around crosshair.");
        if changed {
            self.send_config();
        }
    }

    pub fn sniper_crosshair_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.hud.sniper_crosshair.enabled, "Enable")
            .changed();

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Line Length");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.hud.sniper_crosshair.line_length).range(0.1..=500.0).speed(0.2))
                }).response.changed()
            })
            .inner;

        changed |= ui
            .horizontal(|ui| {
                ui.label("Line Width");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.hud.sniper_crosshair.line_width).range(0.1..=10.0).speed(0.005))
                }).response.changed()
            })
            .inner;

        changed |= ui
            .horizontal(|ui| {
                ui.label("Gap");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.hud.sniper_crosshair.gap).range(0.0..=200.0).speed(0.2))
                }).response.changed()
            })
            .inner;

        if changed {
            self.send_config();
        }
    }

    pub fn grenade_trails_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.hud.grenade_trails, "Enable Trails")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.hud.grenade_predict, "Grenade Predict")
            .changed();

        if changed {
            self.send_config();
        }
    }
}
