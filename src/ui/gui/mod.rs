use egui::{Align2, Color32, CornerRadius, Frame, Margin, Rect, Stroke, Ui};

use crate::{
    config::WeaponConfig,
    cs2::entity::weapon::Weapon,
    message::GameMessage,
    ui::{
        app::{App, ModuleId},
        color::Colors,
    },
};

mod about;
pub mod aimbot;
pub mod bone_selector;
mod helpers;
mod hud;
mod player;
mod r#unsafe;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Aim,
    Player,
    HUD,
    Misc,
}

const TAB_NAMES: &[(Tab, &str)] = &[
    (Tab::Aim, "Aim"),
    (Tab::Player, "Player"),
    (Tab::HUD, "HUD"),
    (Tab::Misc, "Misc"),
];

fn tab_modules(tab: &Tab) -> &'static [(ModuleId, &'static str)] {
    match tab {
        Tab::Aim => &[
            (ModuleId::Aimbot, "AimBot"),
            (ModuleId::Triggerbot, "Triggerbot"),
            (ModuleId::Rcs, "RCS"),
        ],
        Tab::Player => &[(ModuleId::PlayerEsp, "ESP"), (ModuleId::SoundEsp, "Sound ESP")],
        Tab::HUD => &[
            (ModuleId::Hud, "HUD"),
            (ModuleId::BombTimer, "Bomb Timer"),
            (ModuleId::SniperCrosshair, "Sniper Crosshair"),
        ],
        Tab::Misc => &[
            (ModuleId::GrenadeTrails, "Grenade Trails"),
            (ModuleId::FovCircle, "FOV Circle"),
            (ModuleId::Glow, "Glow"),
            (ModuleId::NoFlash, "No Flash"),
            (ModuleId::FovChanger, "FOV Changer"),
            (ModuleId::NoSmoke, "No Smoke"),
        ],
    }
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
        crate::config::write_config(&self.config, &self.current_config);
    }

    fn weapon_config_for(&mut self, tab: &aimbot::AimbotTab, weapon: &Weapon) -> &mut WeaponConfig {
        if *tab == aimbot::AimbotTab::Weapon {
            self.config
                .aim
                .weapons
                .get_mut(weapon)
                .unwrap()
        } else {
            &mut self.config.aim.global
        }
    }

    fn gui(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();
        ctx.set_pixels_per_point(self.display_scale);

        let menu_size = egui::vec2(680.0, 420.0);

        egui::Area::new(egui::Id::new("menu_area"))
            .fixed_pos(self.menu_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let frame = Frame {
                    fill: Colors::WINDOW_BG,
                    stroke: Stroke::new(1.0, Colors::CARD_BORDER_ACTIVE),
                    corner_radius: CornerRadius::same(12),
                    inner_margin: Margin::ZERO,
                    ..Default::default()
                };

                frame.show(ui, |ui| {
                    ui.set_min_size(menu_size);
                    ui.set_max_size(menu_size);

                    Self::tab_bar(ui, &mut self.current_tab);

                    let grid_h = (ui.available_height() - 40.0).max(0.0);
                    let grid_area = ui
                        .allocate_exact_size(
                            egui::vec2(ui.available_width(), grid_h),
                            egui::Sense::hover(),
                        )
                        .0;
                    let grid_content = grid_area.shrink2(egui::vec2(25.0, 20.0));
                    let mut grid_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(grid_content)
                            .layout(egui::Layout::top_down(egui::Align::LEFT)),
                    );
                    {
                        let modules = tab_modules(&self.current_tab);
                        let mut i = 0;
                        while i < modules.len() {
                            grid_ui.horizontal(|ui| {
                                ui.style_mut().spacing.item_spacing = egui::vec2(12.0, 0.0);
                                for _ in 0..3 {
                                    if i < modules.len() {
                                        let (id, name) = modules[i];
                                        if self.detached_module == Some(id) {
                                            ui.allocate_exact_size(
                                                egui::vec2(195.0, 70.0),
                                                egui::Sense::hover(),
                                            );
                                        } else {
                                            self.module_card(ui, id, name);
                                        }
                                        i += 1;
                                    }
                                }
                            });
                            grid_ui.add_space(12.0);
                        }
                    }

                    Self::footer(
                        ui,
                        &mut self.show_settings_popup,
                        &mut self.settings_popup_initialized,
                    );
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
                        let tab_bar_rect =
                            Rect::from_min_size(self.menu_pos, egui::vec2(menu_size.x, 45.0));
                        if tab_bar_rect.contains(pos) && pos.x < self.menu_pos.x + menu_size.x - 60.0
                        {
                            self.menu_pos += delta;
                        }
                    }
                }
            });

        if let Some(module) = self.detached_module {
            render_detached_popup(ctx, self, module);
        }

        if self.show_bone_selector {
            self.bone_selector_popup(ctx);
        }

        if self.show_settings_popup {
            if !self.settings_popup_initialized {
                self.settings_popup_pos = egui::pos2(self.menu_pos.x + 690.0, self.menu_pos.y);
                self.settings_popup_initialized = true;
            }
            render_settings_popup(ctx, self);
        }

        if self.show_about {
            self.about(ctx);
        }
    }

    fn tab_bar(ui: &mut Ui, current_tab: &mut Tab) {
        let height = 45.0;

        let (bar_rect, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), height),
            egui::Sense::hover(),
        );

        let tab_count = TAB_NAMES.len() as f32;
        let tab_width = bar_rect.width() / tab_count;

        let mut tab_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(bar_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        tab_ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);

        for (tab, name) in TAB_NAMES.iter() {
            let is_active = current_tab == tab;

            let (rect, response) = tab_ui.allocate_exact_size(
                egui::vec2(tab_width, height),
                egui::Sense::click(),
            );

            let text_color = if is_active {
                Colors::TAB_ACTIVE
            } else if response.hovered() {
                Colors::TEXT
            } else {
                Colors::TEXT_DISABLED
            };

            let painter = tab_ui.painter_at(rect);
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                *name,
                egui::FontId::proportional(16.0),
                text_color,
            );

            if response.clicked() {
                *current_tab = *tab;
            }
        }

        let painter = ui.painter_at(bar_rect);
        painter.line_segment(
            [
                egui::pos2(bar_rect.left(), bar_rect.bottom()),
                egui::pos2(bar_rect.right(), bar_rect.bottom()),
            ],
            Stroke::new(1.0, Colors::TAB_DIVIDER),
        );

        if let Some(active_idx) = TAB_NAMES.iter().position(|(t, _)| t == current_tab) {
            let active_rect = Rect::from_min_size(
                egui::pos2(bar_rect.left() + tab_width * active_idx as f32, bar_rect.top()),
                egui::vec2(tab_width, height),
            );
            let painter = ui.painter_at(bar_rect);
            painter.line_segment(
                [
                    egui::pos2(active_rect.left() + 8.0, active_rect.bottom()),
                    egui::pos2(active_rect.right() - 8.0, active_rect.bottom()),
                ],
                Stroke::new(2.0, Colors::TAB_ACTIVE),
            );
        }
    }

    fn footer(ui: &mut Ui, show_settings: &mut bool, settings_initialized: &mut bool) {
        let height = 40.0;
        let footer_rect =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), height), egui::Sense::hover())
                .0;

        if ui.is_rect_visible(footer_rect) {
            let painter = ui.painter_at(footer_rect);

            painter.line_segment(
                [
                    egui::pos2(footer_rect.left(), footer_rect.top()),
                    egui::pos2(footer_rect.right(), footer_rect.top()),
                ],
                Stroke::new(1.0, Colors::TAB_DIVIDER),
            );

            painter.text(
                footer_rect.center(),
                Align2::CENTER_CENTER,
                "VibeKill",
                egui::FontId::proportional(18.0),
                Colors::TEXT,
            );

            let gear_rect = Rect::from_center_size(
                egui::pos2(footer_rect.right() - 22.0, footer_rect.center().y),
                egui::vec2(20.0, 20.0),
            );
            let gear_resp = ui.interact(gear_rect, ui.next_auto_id(), egui::Sense::click());
            painter.text(
                gear_rect.center(),
                Align2::CENTER_CENTER,
                "\u{2699}",
                egui::FontId::proportional(14.0),
                if gear_resp.hovered() {
                    Colors::TEXT
                } else {
                    Colors::TEXT_DISABLED
                },
            );
            if gear_resp.clicked() {
                *show_settings = !*show_settings;
                if *show_settings {
                    *settings_initialized = false;
                }
            }
        }
    }

    fn module_card(&mut self, ui: &mut Ui, id: ModuleId, name: &str) {
        let card_size = egui::vec2(195.0, 70.0);
        let card_radius = CornerRadius::same(8);
        let enabled = module_enabled(&self.config, id);

        let (rect, response) = ui.allocate_exact_size(card_size, egui::Sense::click());
        let bg = if response.hovered() {
            Colors::CARD_HOVER
        } else {
            Colors::CARD_BG
        };

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);

            let stroke = if enabled {
                Stroke::new(1.0, Colors::CARD_BORDER_ACTIVE)
            } else {
                Stroke::new(1.0, Colors::CARD_BORDER)
            };
            painter.rect(rect, card_radius, bg, stroke, egui::StrokeKind::Middle);

            let text_pos = egui::pos2(rect.center().x, rect.top() + 26.0);
            painter.text(
                text_pos,
                Align2::CENTER_CENTER,
                name,
                egui::FontId::proportional(14.0),
                if enabled { Colors::TEXT } else { Colors::TEXT_DISABLED },
            );

            // Gear
            let gear_pos = egui::pos2(rect.left() + 12.0, rect.bottom() - 12.0);
            let gear_rect = Rect::from_center_size(gear_pos, egui::vec2(16.0, 16.0));
            let gear_resp = ui.interact(gear_rect, egui::Id::new(("gear", id)), egui::Sense::click());
            painter.text(
                gear_pos,
                Align2::CENTER_CENTER,
                "\u{2699}",
                egui::FontId::proportional(12.0),
                if gear_resp.hovered() {
                    Colors::TEXT
                } else {
                    Colors::TEXT_DISABLED
                },
            );

            // Keybind
            if let Some(ref bind_text) = module_keybind(&self.config, id) {
                let bind_pos = egui::pos2(rect.right() - 12.0, rect.bottom() - 12.0);
                let bind_size = painter.layout_no_wrap(
                    bind_text.clone(),
                    egui::FontId::proportional(10.0),
                    Colors::BIND_TEXT,
                );
                let bind_rect = Rect::from_center_size(
                    bind_pos,
                    egui::vec2(bind_size.size().x + 10.0, 16.0),
                );
                painter.rect(
                    bind_rect,
                    CornerRadius::same(4),
                    Colors::BIND_BG,
                    Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 30)),
                    egui::StrokeKind::Middle,
                );
                painter.text(
                    bind_pos,
                    Align2::CENTER_CENTER,
                    bind_text.clone(),
                    egui::FontId::proportional(10.0),
                    if enabled { Colors::BIND_TEXT } else { Colors::TEXT_DISABLED },
                );
            }

            if response.clicked_by(egui::PointerButton::Secondary) || gear_resp.clicked_by(egui::PointerButton::Secondary) {
                self.detached_module = Some(id);
                self.detached_module_pos =
                    egui::pos2(self.menu_pos.x + 690.0, self.menu_pos.y);
            }

            if response.clicked_by(egui::PointerButton::Primary) || gear_resp.clicked_by(egui::PointerButton::Primary) {
                toggle_module(&mut self.config, id);
                self.send_config();
            }
        }
    }

    pub fn render(&mut self) {
        utils::info!("render: show_menu={}", self.show_menu);
        let self_ptr = self as *mut Self;

        let overlay = match self.overlay.as_mut() {
            Some(o) => o,
            None => {
                utils::info!("render: no overlay");
                return;
            }
        };

        if self.show_menu {
            overlay.ungrab_input();
        }

        let cursor_hittest = self.show_menu;
        let _ = overlay.window().set_cursor_hittest(cursor_hittest);

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

fn module_enabled(config: &crate::config::Config, id: ModuleId) -> bool {
    match id {
        ModuleId::Aimbot => config.aim.global.aimbot.enabled,
        ModuleId::Triggerbot => config.aim.global.triggerbot.enabled,
        ModuleId::Rcs => config.aim.global.rcs.enabled,
        ModuleId::PlayerEsp => config.player.enabled,
        ModuleId::SoundEsp => config.player.sound.enabled,
        ModuleId::Hud => true,
        ModuleId::BombTimer => config.hud.bomb_timer,
        ModuleId::FovCircle => config.hud.fov_circle,
        ModuleId::SniperCrosshair => config.hud.sniper_crosshair.enabled,
        ModuleId::GrenadeTrails => config.hud.grenade_trails,
        ModuleId::Glow => config.misc.glow_enabled,
        ModuleId::NoFlash => config.misc.no_flash,
        ModuleId::FovChanger => config.misc.fov_changer,
        ModuleId::NoSmoke => config.misc.no_smoke,
    }
}

fn toggle_module(config: &mut crate::config::Config, id: ModuleId) {
    match id {
        ModuleId::Aimbot => config.aim.global.aimbot.enabled ^= true,
        ModuleId::Triggerbot => config.aim.global.triggerbot.enabled ^= true,
        ModuleId::Rcs => config.aim.global.rcs.enabled ^= true,
        ModuleId::PlayerEsp => config.player.enabled ^= true,
        ModuleId::SoundEsp => config.player.sound.enabled ^= true,
        ModuleId::Hud => {}
        ModuleId::BombTimer => config.hud.bomb_timer ^= true,
        ModuleId::FovCircle => config.hud.fov_circle ^= true,
        ModuleId::SniperCrosshair => config.hud.sniper_crosshair.enabled ^= true,
        ModuleId::GrenadeTrails => config.hud.grenade_trails ^= true,
        ModuleId::Glow => config.misc.glow_enabled ^= true,
        ModuleId::NoFlash => config.misc.no_flash ^= true,
        ModuleId::FovChanger => config.misc.fov_changer ^= true,
        ModuleId::NoSmoke => config.misc.no_smoke ^= true,
    }
}

fn module_keybind(config: &crate::config::Config, id: ModuleId) -> Option<String> {
    match id {
        ModuleId::Aimbot => Some(format!("{:?}", config.aim.aimbot_hotkey)),
        ModuleId::Triggerbot => Some(format!("{:?}", config.aim.triggerbot_hotkey)),
        ModuleId::PlayerEsp => Some(format!("{:?}", config.player.esp_hotkey)),
        _ => None,
    }
}

fn module_display_name(id: ModuleId) -> &'static str {
    match id {
        ModuleId::Aimbot => "AimBot",
        ModuleId::Triggerbot => "Triggerbot",
        ModuleId::Rcs => "RCS",
        ModuleId::PlayerEsp => "ESP",
        ModuleId::SoundEsp => "Sound ESP",
        ModuleId::Hud => "HUD",
        ModuleId::BombTimer => "Bomb Timer",
        ModuleId::FovCircle => "FOV Circle",
        ModuleId::SniperCrosshair => "Sniper Crosshair",
        ModuleId::GrenadeTrails => "Grenade Trails",
        ModuleId::Glow => "Glow",
        ModuleId::NoFlash => "No Flash",
        ModuleId::FovChanger => "FOV Changer",
        ModuleId::NoSmoke => "No Smoke",
    }
}

fn render_detached_popup(ctx: &egui::Context, app: &mut App, module: ModuleId) {
    egui::Area::new(egui::Id::new("detached_popup"))
        .fixed_pos(app.detached_module_pos)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let frame = Frame {
                fill: Colors::WINDOW_BG,
                stroke: Stroke::new(1.0, Colors::CARD_BORDER_ACTIVE),
                corner_radius: CornerRadius::same(12),
                inner_margin: Margin::ZERO,
                ..Default::default()
            };

            frame.show(ui, |ui| {
                ui.set_max_width(280.0);

                let header_height = 30.0;
                let (header_rect, header_resp) = ui.allocate_exact_size(
                    egui::vec2(280.0, header_height),
                    egui::Sense::click_and_drag(),
                );

                app.detached_module_pos += header_resp.drag_delta();

                if ui.is_rect_visible(header_rect) {
                    let painter = ui.painter_at(header_rect);
                    painter.text(
                        egui::pos2(header_rect.left() + 12.0, header_rect.center().y),
                        Align2::LEFT_CENTER,
                        module_display_name(module),
                        egui::FontId::proportional(13.0),
                        Colors::TEXT,
                    );

                    let close_rect = Rect::from_center_size(
                        egui::pos2(header_rect.right() - 18.0, header_rect.center().y),
                        egui::vec2(24.0, 24.0),
                    );
                    let mut close_child = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(close_rect)
                            .layout(egui::Layout::centered_and_justified(
                                egui::Direction::LeftToRight,
                            )),
                    );
                    let close_resp = close_child
                        .allocate_exact_size(
                            egui::vec2(24.0, 24.0),
                            egui::Sense::click(),
                        )
                        .1;
                    painter.text(
                        close_rect.center(),
                        Align2::CENTER_CENTER,
                        "\u{2715}",
                        egui::FontId::proportional(12.0),
                        if close_resp.hovered() {
                            Colors::CLOSE_HOVER
                        } else {
                            Colors::TEXT_DISABLED
                        },
                    );
                    if close_resp.clicked() {
                        app.detached_module = None;
                    }

                    painter.line_segment(
                        [
                            egui::pos2(header_rect.left(), header_rect.bottom()),
                            egui::pos2(header_rect.right(), header_rect.bottom()),
                        ],
                        Stroke::new(1.0, Colors::POPUP_HEADER),
                    );
                }

                ui.add_space(4.0);
                Frame {
                    fill: Color32::TRANSPARENT,
                    stroke: Stroke::NONE,
                    corner_radius: CornerRadius::ZERO,
                    inner_margin: Margin::symmetric(12, 0),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    render_module_popup_content(ui, app, module);
                });
                ui.add_space(10.0);
            });
        });
}

fn render_module_popup_content(ui: &mut Ui, app: &mut App, id: ModuleId) {
    match id {
        ModuleId::Aimbot => app.aimbot_popup(ui),
        ModuleId::Triggerbot => app.triggerbot_popup(ui),
        ModuleId::Rcs => app.rcs_popup(ui),
        ModuleId::PlayerEsp => app.player_esp_popup(ui),
        ModuleId::SoundEsp => app.sound_esp_popup(ui),
        ModuleId::Hud => app.hud_popup(ui),
        ModuleId::BombTimer => app.bomb_timer_popup(ui),
        ModuleId::FovCircle => app.fov_circle_popup(ui),
        ModuleId::SniperCrosshair => app.sniper_crosshair_popup(ui),
        ModuleId::GrenadeTrails => app.grenade_trails_popup(ui),
        ModuleId::Glow => app.glow_popup(ui),
        ModuleId::NoFlash => app.no_flash_popup(ui),
        ModuleId::FovChanger => app.fov_changer_popup(ui),
        ModuleId::NoSmoke => app.no_smoke_popup(ui),
    }
}

fn render_settings_popup(ctx: &egui::Context, app: &mut App) {
    egui::Area::new(egui::Id::new("settings_popup"))
        .fixed_pos(app.settings_popup_pos)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let frame = Frame {
                fill: Colors::WINDOW_BG,
                stroke: Stroke::new(1.0, Colors::CARD_BORDER_ACTIVE),
                corner_radius: CornerRadius::same(12),
                inner_margin: Margin::ZERO,
                ..Default::default()
            };

            frame.show(ui, |ui| {
                ui.set_min_width(240.0);

                let (header_rect, header_resp) = ui.allocate_exact_size(
                    egui::vec2(240.0, 30.0),
                    egui::Sense::click_and_drag(),
                );

                app.settings_popup_pos += header_resp.drag_delta();

                if ui.is_rect_visible(header_rect) {
                    let painter = ui.painter_at(header_rect);
                    painter.text(
                        egui::pos2(header_rect.left() + 12.0, header_rect.center().y),
                        Align2::LEFT_CENTER,
                        "Settings",
                        egui::FontId::proportional(13.0),
                        Colors::TEXT,
                    );
                    let close_rect = Rect::from_center_size(
                        egui::pos2(header_rect.right() - 18.0, header_rect.center().y),
                        egui::vec2(24.0, 24.0),
                    );
                    let mut close_child = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(close_rect)
                            .layout(egui::Layout::centered_and_justified(
                                egui::Direction::LeftToRight,
                            )),
                    );
                    let close_resp = close_child
                        .allocate_exact_size(
                            egui::vec2(24.0, 24.0),
                            egui::Sense::click(),
                        )
                        .1;
                    painter.text(
                        close_rect.center(),
                        Align2::CENTER_CENTER,
                        "\u{2715}",
                        egui::FontId::proportional(12.0),
                        if close_resp.hovered() {
                            Colors::CLOSE_HOVER
                        } else {
                            Colors::TEXT_DISABLED
                        },
                    );
                    if close_resp.clicked() {
                        app.show_settings_popup = false;
                    }
                    painter.line_segment(
                        [
                            egui::pos2(header_rect.left(), header_rect.bottom()),
                            egui::pos2(header_rect.right(), header_rect.bottom()),
                        ],
                        Stroke::new(1.0, Colors::POPUP_HEADER),
                    );
                }

                ui.add_space(4.0);
                Frame {
                    fill: Color32::TRANSPARENT,
                    stroke: Stroke::NONE,
                    corner_radius: CornerRadius::ZERO,
                    inner_margin: Margin::symmetric(12, 4),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.set_min_width(216.0);

                    if ui.button("Reset Config").clicked() {
                        app.config = crate::config::Config::default();
                        app.send_config();
                    }
                    if ui.button("Config Folder").clicked() {
                        let _ = std::process::Command::new("xdg-open")
                            .arg(crate::config::BASE_PATH.as_os_str())
                            .status();
                    }

                    ui.separator();

                    ui.label("Load Config:");
                    for path in &app.available_configs.clone() {
                        let Some(name) = path.file_name().and_then(|s| s.to_str()).map(|s| s.to_owned()) else {
                            continue;
                        };
                        let is_current = app.current_config == *path;
                        if ui.selectable_label(is_current, &name).clicked() {
                            app.config = crate::config::parse_config(path);
                            app.current_config = path.clone();
                            app.send_config();
                        }
                    }

                    ui.separator();

                    let mut send_stacktraces = app.app_config.send_stacktraces;
                    if ui
                        .checkbox(&mut send_stacktraces, "Send Stacktraces")
                        .changed()
                    {
                        app.app_config.send_stacktraces = send_stacktraces;
                        crate::config::write_app_config(&app.app_config);
                        crate::os::crash::STACKTRACE_SENT
                            .store(!app.app_config.send_stacktraces, std::sync::atomic::Ordering::Relaxed);
                    }

                    ui.separator();

                    if ui.button("Self Destruct").clicked() {
                        app.selfdestruct();
                    }

                    ui.separator();

                    if ui.button("About").clicked() {
                        app.show_about = true;
                        app.show_settings_popup = false;
                    }
                });
                ui.add_space(6.0);
            });
        });
}
