use std::sync::Arc;

use utils::{channel::Channel, log::LoggerOptions, sync::Mutex};

use crate::{
    config::BASE_PATH,
    data::Data,
    os::{crash::install_crash_handler, mouse::check_uinput},
    ui::app::App,
};

mod config;
mod constants;
mod cs2;
mod data;
mod game;
mod math;
mod message;
mod os;
mod parser;
mod ui;

#[cfg(not(target_os = "linux"))]
compile_error!("only linux is supported.");

fn dump_glow() {
    use crate::constants::cs2;
    use crate::cs2::schema::Schema;
    use crate::os::process::Process;

    println!("[dump] opening cs2 process...");
    let Some(process) = Process::open(cs2::PROCESS_NAME) else {
        println!("[dump] could not open cs2 process");
        return;
    };

    let Some(schema_base) = process.module_base_address(cs2::SCHEMA_LIB) else {
        println!("[dump] could not find schema.dll");
        return;
    };

    let Some(schema) = Schema::new(&process, schema_base) else {
        println!("[dump] could not init schema");
        return;
    };

    let scope = schema.get_library(cs2::CLIENT_LIB).unwrap();

    println!("\n[dump] === All glow-related fields ===");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "Glow");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "glow");

    println!("\n[dump] === All classes containing 'Glow' ===");
    scope.dump_class_names("Glow");
    scope.dump_class_names("glow");

    println!("\n[dump] === CGlowProperty fields ===");
    scope.dump_class_all_fields("CGlowProperty");

    println!("\n[dump] === CCSPlayer_GlowServices fields ===");
    if scope.get_class("CCSPlayer_GlowServices").is_some() {
        scope.dump_class_all_fields("CCSPlayer_GlowServices");
    } else {
        println!("[dump]   class 'CCSPlayer_GlowServices' not found");
    }

    println!("\n[dump] === C_BaseModelEntity all fields ===");
    scope.dump_class_all_fields("C_BaseModelEntity");

    println!("\n[dump] === C_CSPlayerPawn all fields ===");
    scope.dump_class_all_fields("C_CSPlayerPawn");

    println!("\n[dump] === C_CSPlayerPawnBase all fields ===");
    scope.dump_class_all_fields("C_CSPlayerPawnBase");
}

fn dump_weapon_schema() {
    use crate::constants::cs2;
    use crate::cs2::schema::Schema;
    use crate::os::process::Process;

    println!("[dump] opening cs2 process...");
    let Some(process) = Process::open(cs2::PROCESS_NAME) else {
        println!("[dump] could not open cs2 process");
        return;
    };
    println!("[dump] pid: {}", process.pid);

    let Some(client_base) = process.module_base_address(cs2::CLIENT_LIB) else {
        println!("[dump] could not find client.dll");
        return;
    };
    println!("[dump] client.dll: 0x{:x}", client_base);

    let Some(schema_base) = process.module_base_address(cs2::SCHEMA_LIB) else {
        println!("[dump] could not find schema.dll");
        return;
    };

    let Some(schema) = Schema::new(&process, schema_base) else {
        println!("[dump] could not init schema");
        return;
    };

    println!("\n[dump] === Weapon classes ===");
    schema.dump_weapon_classes(cs2::CLIENT_LIB);

    println!("\n[dump] === Searching for spread fields ===");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "Spread");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "spread");

    println!("\n[dump] === Searching for accuracy fields ===");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "Accuracy");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "accuracy");

    println!("\n[dump] === Searching for WeaponVData ===");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "VData");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "vdata");

    println!("\n[dump] === All fields with 'VData' in name ===");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "VData");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "vdata");

    println!("\n[dump] === All pointer fields (m_p/m_h) on C_CSWeaponBase ===");
    let scope = schema.get_library(cs2::CLIENT_LIB).unwrap();
    scope.dump_class_pointer_fields("C_CSWeaponBase");
    scope.dump_class_pointer_fields("C_EconEntity");
    scope.dump_class_pointer_fields("C_AttributeContainer");
    scope.dump_class_pointer_fields("C_EconItemView");

    println!("\n[dump] === All fields on C_EconEntity ===");
    scope.dump_class_all_fields("C_EconEntity");
    println!("\n[dump] === All fields on C_AttributeContainer ===");
    scope.dump_class_all_fields("C_AttributeContainer");
    println!("\n[dump] === All fields on C_EconItemView ===");
    scope.dump_class_all_fields("C_EconItemView");

    println!("\n[dump] === Searching for VData/Schema/DataTable refs on weapon classes ===");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "WeaponVData");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "weapon_vdata");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "m_szVData");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "m_pVData");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "m_pWeaponData");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "m_pCSWeapon");
    schema.dump_all_field_names(cs2::CLIENT_LIB, "m_pGameSceneNode");

    println!("\n[dump] === Dumping ALL classes with 'VData' fields ===");
    scope.dump_all_classes_with_field("VData");
    scope.dump_all_classes_with_field("m_pWeaponData");

    println!("\n[dump] === Searching for AttributeList class ===");
    scope.dump_class_names("Attribute");
    scope.dump_class_names("attribute");

    println!("\n[dump] === CAttributeList fields ===");
    scope.dump_class_all_fields("CAttributeList");

    println!("\n[dump] === Searching for weapon data systems ===");
    scope.dump_class_names("WeaponSystem");
    scope.dump_class_names("WeaponData");
    scope.dump_class_names("weapon_data");

    println!("\n[dump] === C_CSWeaponBaseGun fields ===");
    scope.dump_class_all_fields("C_CSWeaponBaseGun");

    println!("\n[dump] === Dumping ALL field names containing 'Data' being m_p/m_h ===");
    scope.dump_all_fields_with_condition("Data", true);

    println!("\n[dump] === Dumping CBodyComponentSkeletonInstance ===");
    scope.dump_class_all_fields("CBodyComponentSkeletonInstance");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--dump-weapon") {
        dump_weapon_schema();
        return;
    }
    if args.iter().any(|a| a == "--dump-glow") {
        dump_glow();
        return;
    }

    utils::log::init(
        LoggerOptions::default()
            .file(BASE_PATH.join("deadlocked.log"))
            .truncate(true),
        |w, rec| {
            writeln!(
                w,
                "[{}] [{}:{}] {}",
                rec.level, rec.location.file, rec.location.line, rec.args
            )
        },
    )
    .expect("failed to initialize logger");

    if !check_uinput() {
        return;
    }

    install_crash_handler();

    // this runs as x11 for now, because wayland decorations for winit are not good
    // and don't support disabling the maximize button
    unsafe { std::env::remove_var("WAYLAND_DISPLAY") };

    let (channel_gui, channel_game) = Channel::new();
    let data = Arc::new(Mutex::new(Data::default()));
    let data_game = data.clone();

    std::thread::spawn(move || {
        install_crash_handler();
        game::GameManager::new(channel_game, data_game).run();
    });

    let event_loop = match winit::event_loop::EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(err) => {
            utils::error!("failed to create event loop: {err}");
            return;
        }
    };
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::new(channel_gui, data);
    event_loop.run_app(&mut app).unwrap();
}
