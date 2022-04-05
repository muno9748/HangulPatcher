#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod history_type;
mod win_overlay;
mod char_type;
mod settings;
mod patcher;
mod korean;
mod win32;
mod keys;

use std::{sync::atomic::{AtomicBool, Ordering}};

use patcher::Patcher;
use settings::Settings;
use tauri::Manager;

#[cfg(not(target_os = "windows"))]
compile_error!("Currently, Korean Patcher only supports upper windows 10. Please delete this when you're going to support other os.");

static IS_RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    let settings = Box::leak(Box::new(Settings::new()));

    Patcher::set_instance(Patcher::new(settings));

    let app = tauri::Builder::default()
        .build(tauri::generate_context!("./tauri.conf.json"))
        .expect("Unknown error occured while creating app context!");

    app.listen_global("load", |ev| {
        let data: Vec<&str> = ev.payload().unwrap().split(",").collect();

        settings.chat_open_key.store(data[0].parse().unwrap(), Ordering::Relaxed);
        settings.korean_toggle_key.store(data[1].parse().unwrap(), Ordering::Relaxed);
        settings.block_kr_toggle_ingame.store(data[2] == "1", Ordering::Relaxed);
        settings.auto_disable_korean.store(data[3] == "1", Ordering::Relaxed);
        settings.show_overlay.store(data[4] == "1", Ordering::Relaxed);

        if data[4] == "1" {
            Patcher::run_overlay();
        }
        
        Patcher::start();
    });

    app.listen_global("set_setting", |ev| {
        let data: Vec<&str> = ev.payload().unwrap().split(",").collect();

        match data[0].parse::<u8>().unwrap() {
            0 => settings.chat_open_key.store(data[1].parse().unwrap(), Ordering::Relaxed),
            1 => settings.korean_toggle_key.store(data[1].parse().unwrap(), Ordering::Relaxed),
            2 => settings.block_kr_toggle_ingame.store(data[1] == "1", Ordering::Relaxed),
            3 => settings.auto_disable_korean.store(data[1] == "1", Ordering::Relaxed),
            4 => {
                if settings.show_overlay.load(Ordering::SeqCst) {
                    if data[1] == "0" {
                        Patcher::stop_overlay();
                    }
                } else if data[1] == "1" {
                    Patcher::run_overlay();
                }

                settings.show_overlay.store(data[1] == "1", Ordering::SeqCst);
            }
            _ => panic!("Invalid Operation")
        }
    });

    app.listen_global("run", |_| {
        if !IS_RUNNING.load(Ordering::Relaxed) {
            if settings.show_overlay.load(Ordering::Relaxed) {
                Patcher::run_overlay();
            }

            Patcher::start();

            IS_RUNNING.store(true, Ordering::Relaxed);
        }
    });

    app.listen_global("stop", |_| {
        if IS_RUNNING.load(Ordering::Relaxed) {
            if settings.show_overlay.load(Ordering::Relaxed) {
                Patcher::stop_overlay();
            }

            Patcher::stop();

            IS_RUNNING.store(false, Ordering::Relaxed);
        }
    });

    app.run(|_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            if IS_RUNNING.load(Ordering::Relaxed) {
                if settings.show_overlay.load(Ordering::Relaxed) {
                    Patcher::stop_overlay();
                }

                Patcher::stop();

                IS_RUNNING.store(false, Ordering::Relaxed);
            }
        }
        _ => {}
    });
}