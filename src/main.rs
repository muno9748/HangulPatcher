#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod win_overlay;
mod settings;
mod patcher;
mod buffer;
mod win32;
mod keys;

use std::sync::{atomic::{AtomicBool, Ordering}, mpsc, Mutex};

use patcher::Patcher;
use settings::Settings;
use tauri::Manager;
use win32 as os;

#[cfg(not(target_os = "windows"))]
compile_error!("Currently, Korean Patcher only supports upper windows 10. Please delete this when you're going to support other os.");

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

enum EventType {
    Load,
    SetSetting,
    Run,
    Stop,
    Exit
}

pub(crate) struct Event {
    pub ev: EventType,
    pub payload: Option<String>
}

fn main() {
    let settings = Box::leak(Box::new(Settings::new()));
    let (tx, rx) = mpsc::channel::<()>();
    let tx = Box::leak(Box::new(Mutex::new(tx)));

    std::thread::spawn(|| {
        let tx = tx.lock().unwrap();

        Patcher::new(settings).set_instance();
    
        os::msg_loop(|ev| unsafe {
            match ev.ev {
                EventType::Load => {
                    let data: String = ev.payload.clone().unwrap_unchecked();
                    let data: Vec<&str> = data.split(",").collect();
    
                    settings.chat_open_key.store(data[0].parse().unwrap(), Ordering::Relaxed);
                    settings.korean_toggle_key.store(data[1].parse().unwrap(), Ordering::Relaxed);
                    settings.block_kr_toggle_ingame.store(data[2] == "1", Ordering::Relaxed);
                    settings.auto_disable_korean.store(data[3] == "1", Ordering::Relaxed);
                    settings.show_overlay.store(data[4] == "1", Ordering::Relaxed);
            
                    if data[4] == "1" {
                        Patcher::run_overlay();
                    }
                    
                    Patcher::start();
            
                    IS_RUNNING.store(true, Ordering::Relaxed);
                }
                EventType::SetSetting => {
                    let data: String = ev.payload.clone().unwrap_unchecked();
                    let data: Vec<&str> = data.split(",").collect();

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
                }
                EventType::Run => {
                    if !IS_RUNNING.load(Ordering::Relaxed) {
                        if settings.show_overlay.load(Ordering::Relaxed) {
                            Patcher::run_overlay();
                        }

                        Patcher::start();

                        IS_RUNNING.store(true, Ordering::Relaxed);
                    }
                }
                EventType::Stop => {
                    if IS_RUNNING.load(Ordering::Relaxed) {
                        if settings.show_overlay.load(Ordering::Relaxed) {
                            Patcher::stop_overlay();
                        }
            
                        Patcher::stop();
            
                        IS_RUNNING.store(false, Ordering::Relaxed);
                    }
                }
                EventType::Exit => {
                    if IS_RUNNING.load(Ordering::Relaxed) {
                        if settings.show_overlay.load(Ordering::Relaxed) {
                            Patcher::stop_overlay();
                        }
            
                        Patcher::stop();
            
                        IS_RUNNING.store(false, Ordering::Relaxed);
                    }

                    tx.send(()).unwrap();
                }
            }
        });
    });

    let app = tauri::Builder::default()
        .build(tauri::generate_context!("./tauri.conf.json"))
        .expect("Unknown error occured while creating app context!");

    app.listen_global("load", move |ev| {
        os::send_msg(Event {
            ev: EventType::Load,
            payload: Some(ev.payload().unwrap().to_string())
        });
    });

    app.listen_global("set_setting", move |ev| {
        os::send_msg(Event {
            ev: EventType::SetSetting,
            payload: Some(ev.payload().unwrap().to_string())
        });
    });
    
    app.listen_global("run", move |_| {
        os::send_msg(Event {
            ev: EventType::Run,
            payload: None
        });
    });
    
    app.listen_global("stop", move |_| {
        os::send_msg(Event {
            ev: EventType::Stop,
            payload: None
        });
    });

    app.run(move |_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            os::send_msg(Event {
                ev: EventType::Exit,
                payload: None
            });
            
            rx.recv().unwrap();
        }
        _ => {}
    });
}