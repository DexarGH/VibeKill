use std::hash::Hash;

use egui::{Event, Sense, Ui, Widget};

use crate::cs2::key_codes::KeyCode;

pub fn keybind(ui: &mut Ui, id: &str, label: &str, keycode: &mut KeyCode) -> bool {
    ui.horizontal(|ui| {
        let res = ui.add(Keybind::new(keycode, id));
        ui.label(label);
        res
    })
    .inner
    .changed()
}

pub struct Keybind<'gui> {
    keycode: &'gui mut KeyCode,
    id: egui::Id,
}

impl<'gui> Keybind<'gui> {
    pub fn new(keycode: &'gui mut KeyCode, id: impl Hash) -> Self {
        Self {
            keycode,
            id: egui::Id::new(id),
        }
    }
}

impl<'gui> Widget for Keybind<'gui> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let listening_id = ui.make_persistent_id(self.id);

        let mut listening = {
            let ctx = ui.ctx();
            ctx.memory(|mem| mem.data.get_temp::<bool>(listening_id).unwrap_or(false))
        };

        let text = if listening {
            "...".to_string()
        } else {
            format!("{:?}", self.keycode)
        };

        let mut response = ui.button(text);

        if response.clicked() {
            listening = !listening;
        }

        if response.secondary_clicked() {
            listening = false;
        }

        if listening {
            let input = ui.input(|i| {
                for event in &i.events {
                    if let Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } = event
                    {
                        dbg!(key);
                        if *key == egui::Key::F35 {
                            return KeyCode::from_egui_modifiers(*modifiers);
                        } else {
                            return KeyCode::from_egui(*key);
                        }
                    }

                    if let Event::PointerButton {
                        button,
                        pressed: true,
                        ..
                    } = event
                    {
                        return Some(KeyCode::from_egui_mouse(*button));
                    }
                }
                None
            });

            if let Some(input) = input {
                if input != KeyCode::Escape {
                    *self.keycode = input;
                    response.mark_changed();
                }
                listening = false;
            }
        }

        let ctx = ui.ctx();
        ctx.memory_mut(|mem| mem.data.insert_temp(listening_id, listening));

        response
    }
}
