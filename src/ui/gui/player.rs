use egui::{DragValue, Ui};

use crate::ui::{app::App, gui::helpers::keybind};

impl App {
    pub fn player_esp_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        if keybind(ui, "esp_key", "Hotkey", &mut self.config.player.esp_hotkey) {
            changed = true;
        }

        changed |= ui
            .checkbox(&mut self.config.player.enabled, "Enable")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.show_friendlies, "Show Friendlies")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.head_circle, "Head Circle")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.visible_only, "Visible Only")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.offscreen_arrows, "Offscreen Arrows")
            .changed();

        ui.add_space(4.0);

        {
            let p = &mut self.config.player;
            let box_modes = ["None", "Health", "Color"];
            let box_idx = match &p.draw_box {
                crate::config::DrawMode::None => 0,
                crate::config::DrawMode::Health => 1,
                crate::config::DrawMode::Color => 2,
            };
            ui.horizontal(|ui| {
                ui.label("Box");
                egui::ComboBox::new("esp_box", "")
                    .selected_text(box_modes[box_idx])
                    .show_ui(ui, |ui| {
                        for (i, m) in box_modes.iter().enumerate() {
                            if ui.selectable_label(box_idx == i, *m).clicked() {
                                p.draw_box = match i {
                                    0 => crate::config::DrawMode::None,
                                    1 => crate::config::DrawMode::Health,
                                    _ => crate::config::DrawMode::Color,
                                };
                                changed = true;
                            }
                        }
                    });
            });
        }

        ui.add_space(4.0);

        changed |= ui
            .checkbox(&mut self.config.player.health_bar, "Health Bar")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.armor_bar, "Armor Bar")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.player_name, "Player Name")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.weapon_icon, "Weapon Icon")
            .changed();
        changed |= ui
            .checkbox(&mut self.config.player.tags, "Show Tags")
            .changed();

        if changed {
            self.send_config();
        }
    }

    pub fn sound_esp_popup(&mut self, ui: &mut Ui) {
        let mut changed = false;

        changed |= ui
            .checkbox(&mut self.config.player.sound.enabled, "Enable")
            .changed();

        ui.add_space(4.0);

        changed |= ui
            .horizontal(|ui| {
                ui.label("Fadeout (s)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(DragValue::new(&mut self.config.player.sound.fadeout_duration).range(0.0..=10.0).speed(0.01))
                }).response.changed()
            })
            .inner;

        ui.add_space(4.0);

        changed |= ui
            .checkbox(&mut self.config.player.sound.show_visible, "Show Visible")
            .changed();

        if changed {
            self.send_config();
        }
    }
}
