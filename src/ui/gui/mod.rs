use egui::{Align, Ui};

use crate::{
    config::{WeaponConfig, write_config},
    message::{GameMessage, GameStatus},
    ui::{app::App, color::Colors, gui::aimbot::AimbotTab},
};

mod about;
pub mod aimbot;
mod application;
mod config;
mod grenade;
mod helpers;
mod hud;
mod player;
mod r#unsafe;

#[derive(PartialEq)]
pub enum Tab {
    Aimbot,
    Player,
    Hud,
    Grenades,
    Unsafe,
    Config,
    Application,
}

impl App {
    pub fn send_config(&self) {
        self.send_message(GameMessage(Box::new(self.config.clone())));
        self.save();
    }

    pub fn send_message(&self, message: GameMessage) {
        if self.channel.send(message).is_err() {
            std::process::exit(1);
        }
    }

    fn save(&self) {
        write_config(&self.config, &self.current_config);
    }

    fn gui(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();
        ctx.set_pixels_per_point(self.display_scale);

        egui::Area::new(egui::Id::new("menu_area"))
            .fixed_pos(self.menu_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let frame = egui::Frame::window(ui.style());
                let mut title_bar_rect = egui::Rect::NOTHING;
                frame.show(ui, |ui| {
                    ui.set_min_size(egui::vec2(750.0, 450.0));
                    egui::Panel::top("menu_bar")
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            title_bar_rect = ui.min_rect();
                            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                if ui.button("Selfdestruct").clicked() {
                                    self.selfdestruct();
                                }
                            });
                        });

                    egui::Panel::left("sidebar")
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.selectable_value(&mut self.current_tab, Tab::Aimbot, "\u{f04fe} Aimbot");
                            ui.selectable_value(&mut self.current_tab, Tab::Player, "\u{f0013} Player");
                            ui.selectable_value(&mut self.current_tab, Tab::Hud, "\u{f0379} Hud");
                            ui.selectable_value(&mut self.current_tab, Tab::Grenades, "\u{f0691} Grenades");
                            ui.selectable_value(&mut self.current_tab, Tab::Unsafe, "\u{f0ce6} Unsafe");
                            ui.selectable_value(&mut self.current_tab, Tab::Config, "\u{f168b} Config");
                            ui.selectable_value(
                                &mut self.current_tab,
                                Tab::Application,
                                "\u{f1577} Application",
                            );

                            ui.with_layout(egui::Layout::bottom_up(Align::Min), |ui| {
                                if ui.button("Report Issue").clicked() {
                                    let _ = std::process::Command::new("xdg-open")
                                        .arg("https://github.com/avitran0/deadlocked/issues")
                                        .status();
                                }

                                if ui.button("About").clicked() {
                                    self.show_about = true;
                                }

                                ui.label(egui::RichText::new(format!("{}", self.game_status)).color(
                                    match self.game_status {
                                        GameStatus::Working => Colors::GREEN,
                                        GameStatus::NotStarted => Colors::YELLOW,
                                    },
                                ));
                            });
                        });

                    egui::CentralPanel::default().show_inside(ui, |ui| match self.current_tab {
                        Tab::Aimbot => self.aimbot_settings(ui),
                        Tab::Player => self.player_settings(ui),
                        Tab::Hud => self.hud_settings(ui),
                        Tab::Grenades => self.grenade_settings(ui),
                        Tab::Unsafe => self.unsafe_settings(ui),
                        Tab::Config => self.config_settings(ui),
                        Tab::Application => self.application_settings(ui),
                    });

                    if self.show_about {
                        self.about(ui.ctx());
                    }

                    if self.app_config.first_launch {
                        self.stacktrace_popup(ui.ctx());
                    }
                });
                let (primary_down, delta, interact_pos) = ctx.input(|i| {
                    (
                        i.pointer.button_down(egui::PointerButton::Primary),
                        i.pointer.delta(),
                        i.pointer.interact_pos(),
                    )
                });
                if primary_down && delta != egui::Vec2::ZERO {
                    if let Some(pos) = interact_pos {
                        if title_bar_rect.contains(pos) {
                            self.menu_pos += delta;
                        }
                    }
                }
            });
    }

    fn weapon_config(&mut self) -> &mut WeaponConfig {
        if self.aimbot_tab == AimbotTab::Weapon {
            self.config
                .aim
                .weapons
                .get_mut(&self.aimbot_weapon)
                .unwrap()
        } else {
            &mut self.config.aim.global
        }
    }

    pub fn render(&mut self) {
        let self_ptr = self as *mut Self;

        let overlay = self.overlay.as_mut().unwrap();

        let cursor_hittest = self.show_menu;
        overlay.window().set_cursor_hittest(cursor_hittest).unwrap();

        if let Err(err) = overlay.make_current() {
            utils::error!("could not make overlay window current: {err}");
            return;
        }

        overlay.run(move |ui| {
            (unsafe { &mut *self_ptr }).overlay(ui);
            if (unsafe { &mut *self_ptr }).show_menu {
                (unsafe { &mut *self_ptr }).gui(ui);
            }
        });
        overlay.clear();
        overlay.paint();

        if let Err(err) = overlay.swap_buffers() {
            utils::error!("could not swap overlay window buffers: {err}");
        }
    }
}
