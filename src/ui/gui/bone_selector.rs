use egui::{Align2, CornerRadius, Frame, Margin, Rect, Stroke, Vec2};
use strum::IntoEnumIterator;

use crate::{
    cs2::bones::Bones,
    ui::{app::App, color::Colors},
};

const TPOSE: [(f32, f32); 19] = [
    (0.50, 0.03),
    (0.50, 0.12),
    (0.50, 0.19),
    (0.50, 0.27),
    (0.50, 0.35),
    (0.50, 0.43),
    (0.50, 0.52),
    (0.27, 0.14),
    (0.13, 0.22),
    (0.05, 0.30),
    (0.73, 0.14),
    (0.87, 0.22),
    (0.95, 0.30),
    (0.43, 0.59),
    (0.39, 0.74),
    (0.37, 0.92),
    (0.57, 0.59),
    (0.61, 0.74),
    (0.63, 0.92),
];

fn bone_idx(bone: Bones) -> usize {
    match bone {
        Bones::Head => 0,
        Bones::Neck => 1,
        Bones::Spine4 => 2,
        Bones::Spine3 => 3,
        Bones::Spine2 => 4,
        Bones::Spine1 => 5,
        Bones::Hip => 6,
        Bones::LeftShoulder => 7,
        Bones::LeftElbow => 8,
        Bones::LeftHand => 9,
        Bones::RightShoulder => 10,
        Bones::RightElbow => 11,
        Bones::RightHand => 12,
        Bones::LeftHip => 13,
        Bones::LeftKnee => 14,
        Bones::LeftFoot => 15,
        Bones::RightHip => 16,
        Bones::RightKnee => 17,
        Bones::RightFoot => 18,
    }
}

fn tpose_screen(rect: Rect, idx: usize) -> egui::Pos2 {
    let (nx, ny) = TPOSE[idx];
    egui::pos2(
        rect.left() + nx * rect.width(),
        rect.top() + ny * rect.height(),
    )
}

impl App {
    pub fn bone_selector_popup(&mut self, ctx: &egui::Context) {
        if self.bone_selector_pos == egui::Pos2::ZERO {
            self.bone_selector_pos = egui::pos2(400.0, 200.0);
        }

        egui::Area::new(egui::Id::new("bone_selector"))
            .fixed_pos(self.bone_selector_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                Frame {
                    fill: Colors::WINDOW_BG,
                    stroke: Stroke::new(1.0, Colors::CARD_BORDER_ACTIVE),
                    corner_radius: CornerRadius::same(12),
                    inner_margin: Margin::ZERO,
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.set_min_width(280.0);
                    ui.set_max_width(280.0);

                    let header_height = 30.0;
                    let (header_rect, header_resp) = ui.allocate_exact_size(
                        Vec2::new(280.0, header_height),
                        egui::Sense::click_and_drag(),
                    );
                    self.bone_selector_pos += header_resp.drag_delta();

                    if ui.is_rect_visible(header_rect) {
                        let painter = ui.painter_at(header_rect);
                        painter.text(
                            egui::pos2(header_rect.left() + 12.0, header_rect.center().y),
                            Align2::LEFT_CENTER,
                            "Bone Selector",
                            egui::FontId::proportional(13.0),
                            Colors::TEXT,
                        );
                        let close_rect = Rect::from_center_size(
                            egui::pos2(header_rect.right() - 18.0, header_rect.center().y),
                            Vec2::new(24.0, 24.0),
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
                                Vec2::new(24.0, 24.0),
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
                            self.show_bone_selector = false;
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

                    let canvas_size = Vec2::new(260.0, 330.0);
                    let (canvas_rect, _) = ui.allocate_exact_size(canvas_size, egui::Sense::hover());

                    if ui.is_rect_visible(canvas_rect) {
                        let painter = ui.painter_at(canvas_rect);
                        let mut changed = false;

                        let line_stroke = Stroke::new(1.5, Colors::TEXT_DISABLED);
                        for (a, b) in &Bones::CONNECTIONS {
                            painter.line_segment(
                                [
                                    tpose_screen(canvas_rect, bone_idx(*a)),
                                    tpose_screen(canvas_rect, bone_idx(*b)),
                                ],
                                line_stroke,
                            );
                        }

                        let mut hovered_bone: Option<Bones> = None;

                        for bone in Bones::iter() {
                            let idx = bone_idx(bone);
                            let pos = tpose_screen(canvas_rect, idx);
                            let node_rect =
                                Rect::from_center_size(pos, Vec2::splat(20.0));

                            let mut child_ui = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(node_rect)
                                    .layout(egui::Layout::centered_and_justified(
                                        egui::Direction::LeftToRight,
                                    )),
                            );
                            let resp = child_ui
                                .allocate_exact_size(
                                    Vec2::splat(20.0),
                                    egui::Sense::click(),
                                )
                                .1;

                            if resp.clicked() {
                                let tab = self.aimbot_tab.clone();
                                let weapon = self.aimbot_weapon.clone();
                                let cfg = self.weapon_config_for(&tab, &weapon);
                                if cfg.aimbot.bones.iter().any(|b| *b == bone) {
                                    cfg.aimbot.bones.retain(|b| *b != bone);
                                } else {
                                    cfg.aimbot.bones.push(bone);
                                }
                                changed = true;
                            }

                            if resp.hovered() {
                                hovered_bone = Some(bone);
                            }
                        }

                        {
                            let tab = self.aimbot_tab.clone();
                            let weapon = self.aimbot_weapon.clone();
                            let cfg = self.weapon_config_for(&tab, &weapon);
                            for bone in Bones::iter() {
                                let idx = bone_idx(bone);
                                let pos = tpose_screen(canvas_rect, idx);
                                let is_selected =
                                    cfg.aimbot.bones.iter().any(|b| *b == bone);

                                if is_selected {
                                    painter.circle_filled(pos, 6.0, Colors::TAB_ACTIVE);
                                } else {
                                    painter.circle_stroke(
                                        pos,
                                        6.0,
                                        Stroke::new(1.5, Colors::TEXT_DISABLED),
                                    );
                                }

                                if Some(bone) == hovered_bone {
                                    painter.text(
                                        egui::pos2(pos.x, pos.y - 10.0),
                                        Align2::CENTER_BOTTOM,
                                        &format!("{}", bone),
                                        egui::FontId::proportional(10.0),
                                        Colors::TEXT,
                                    );
                                }
                            }
                        }

                        if changed {
                            self.send_config();
                        }
                    }

                    ui.add_space(4.0);

                    {
                        let tab = self.aimbot_tab.clone();
                        let weapon = self.aimbot_weapon.clone();
                        let cfg = self.weapon_config_for(&tab, &weapon);
                        let total = Bones::iter().count();
                        let count = cfg.aimbot.bones.len();
                        let mut changed = false;

                        ui.horizontal(|ui| {
                            ui.label(format!("Bones: {}/{}", count, total));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("None").clicked() {
                                        cfg.aimbot.bones.clear();
                                        changed = true;
                                    }
                                    if ui.button("All").clicked() {
                                        cfg.aimbot.bones = Bones::iter().collect();
                                        changed = true;
                                    }
                                },
                            );
                        });

                        if changed {
                            let _ = cfg;
                            self.send_config();
                        }
                    }

                    ui.add_space(8.0);
                });
            });
    }
}
