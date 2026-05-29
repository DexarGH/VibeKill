use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, atomic::Ordering},
    time::{Duration, Instant},
};

use utils::{channel::Channel, sync::Mutex};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, StartCause, WindowEvent},
    keyboard::{Key, NamedKey},
};

use crate::{
    config::{
        ApplicationConfig, CONFIG_PATH, Config, DEFAULT_CONFIG_NAME, available_configs,
        parse_config, read_app_config, write_config,
    },
    cs2::entity::weapon::Weapon,
    data::{Data, SoundType},
    message::{GameMessage, GameStatus, UiMessage},
    os::crash::STACKTRACE_SENT,
    ui::{
        grenades::{Grenade, GrenadeList, read_grenades},
        gui::{Tab, aimbot::AimbotTab},
        trail::Trail,
        window_context::WindowContext,
    },
};

pub struct App {
    pub overlay: Option<WindowContext>,
    next_frame_time: Instant,
    pub show_about: bool,
    pub show_menu: bool,

    pub channel: Channel<GameMessage, UiMessage>,
    pub data: Arc<Mutex<Data>>,

    pub game_status: GameStatus,
    pub display_scale: f32,
    pub trails: HashMap<u64, Trail>,
    pub player_sounds: HashMap<u64, (Instant, SoundType)>,

    pub grenades: GrenadeList,
    pub new_grenade: Grenade,
    pub current_grenade: Option<(String, usize)>,

    pub app_config: ApplicationConfig,
    pub config: Config,
    pub current_config: PathBuf,
    pub available_configs: Vec<PathBuf>,
    pub new_config_name: String,

    pub current_tab: Tab,
    pub aimbot_tab: AimbotTab,
    pub aimbot_weapon: Weapon,
}

impl App {
    pub fn new(channel: Channel<GameMessage, UiMessage>, data: Arc<Mutex<Data>>) -> Self {
        let config = parse_config(&CONFIG_PATH.join(DEFAULT_CONFIG_NAME));
        write_config(&config, &CONFIG_PATH.join(DEFAULT_CONFIG_NAME));
        let grenades = read_grenades();

        let app_config = read_app_config();

        if !app_config.first_launch && !app_config.send_stacktraces {
            STACKTRACE_SENT.store(true, Ordering::Relaxed);
        }

        let ret = Self {
            overlay: None,

            next_frame_time: Instant::now() + Duration::from_millis(16),
            show_about: false,
            show_menu: false,

            channel,
            data,

            app_config,
            config,
            current_config: CONFIG_PATH.join(DEFAULT_CONFIG_NAME),
            available_configs: available_configs(),
            new_config_name: String::new(),

            game_status: GameStatus::NotStarted,
            display_scale: 1.0,
            trails: HashMap::new(),
            player_sounds: HashMap::new(),

            grenades,
            new_grenade: Grenade::new(),
            current_grenade: None,

            current_tab: Tab::Aimbot,
            aimbot_tab: AimbotTab::Global,
            aimbot_weapon: Weapon::Ak47,
        };
        ret.send_config();
        ret
    }

    fn create_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.overlay = Some(WindowContext::new(event_loop, true, self.config.accent_color));
    }

    fn frame_duration(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.config.fps as f32)
    }

    pub fn selfdestruct(&self) {
        std::process::exit(0);
    }
}

impl ApplicationHandler for App {
    fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, cause: StartCause) {
        if let StartCause::ResumeTimeReached { .. } = cause {
            self.next_frame_time += self.frame_duration();

            let now = Instant::now();
            if self.next_frame_time < now {
                self.next_frame_time = now + self.frame_duration();
            }

            self.render();

            event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                self.next_frame_time,
            ));
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop);

        self.next_frame_time = Instant::now() + self.frame_duration();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
            self.next_frame_time,
        ));
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        window_event: WindowEvent,
    ) {
        while let Ok(message) = self.channel.try_receive() {
            match message {
                UiMessage::Status(status) => self.game_status = status,
                UiMessage::ToggleMenu => self.show_menu = !self.show_menu,
            }
        }

        let Some(overlay) = &mut self.overlay else {
            return;
        };

        if overlay.window().id() != window_id {
            return;
        }

        match &window_event {
            WindowEvent::CloseRequested => self.selfdestruct(),
            WindowEvent::Resized(new_size) => {
                overlay.resize(*new_size);
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                if let Key::Named(key) = &event.logical_key {
                    let modifiers = match key {
                        NamedKey::Control => Some(egui::Modifiers::CTRL),
                        NamedKey::Shift => Some(egui::Modifiers::SHIFT),
                        NamedKey::Alt => Some(egui::Modifiers::ALT),
                        NamedKey::Insert => {
                            if event.state == ElementState::Pressed && !event.repeat {
                                self.show_menu = !self.show_menu;
                            }
                            None
                        }
                        _ => None,
                    };

                    if let Some(modifiers) = modifiers {
                        overlay.process_modifier(
                            modifiers,
                            event.state == ElementState::Pressed,
                            event.repeat,
                        );
                    }
                }
                if self.show_menu {
                    let _ = overlay.process_event(&window_event);
                }
            }
            _ => {
                if self.show_menu {
                    let _ = overlay.process_event(&window_event);
                }
            }
        }
    }
}
