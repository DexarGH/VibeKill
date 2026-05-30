use egui::{DragValue, Ui};

use crate::ui::app::App;

impl App {
    pub fn glow_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.misc.glow_enabled, "Enable Glow")
            .changed();

        if changed {
            self.send_config();
        }
    }

    pub fn no_flash_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.misc.no_flash, "No Flash")
            .changed();

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Max Flash Alpha");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.misc.max_flash_alpha).range(0.0..=255.0).speed(0.5))
                }).response.changed()
            })
            .inner;

        if changed {
            self.send_config();
        }
    }

    pub fn fov_changer_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.misc.fov_changer, "FOV Changer")
            .changed();

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Desired FOV");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.misc.desired_fov).range(1..=179).speed(0.1))
                }).response.changed()
            })
            .inner;

        if changed {
            self.send_config();
        }
    }

    pub fn no_smoke_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.misc.no_smoke, "No Smoke")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.misc.change_smoke_color, "Change Smoke Color")
            .changed();

        if changed {
            self.send_config();
        }
    }
}
