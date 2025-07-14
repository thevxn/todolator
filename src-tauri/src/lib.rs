mod commands;
mod tasks;

use std::{
    sync::Mutex,
    thread::{self},
    time::Duration,
};

use chrono::Utc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    App, AppHandle, Emitter, Listener, Manager,
};

use crate::{
    commands::{close, complete_task, create_task, get_next_task, get_tasks, maximize, minimize},
    tasks::TaskReminder,
};

fn setup_tray(app: &mut App) {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                println!("double left-click pressed");

                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            _ => {
                // println!("unhandled event {event:?}");
            }
        })
        .build(app)
        .unwrap();
}

fn spawn_reminder(handle: AppHandle, task_name: String, label: String) {
    std::thread::spawn(move || {
        WebviewWindowBuilder::new(
            &handle,
            label,
            tauri::WebviewUrl::App("reminder.html".into()),
        )
        .title(task_name)
        .decorations(false)
        .resizable(true)
        .center()
        .focused(true)
        .drag_and_drop(true)
        .inner_size(640.00, 250.00)
        .build()
        .unwrap();
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            setup_tray(app);
            let handle = app.handle().clone();
            app.listen("state-changed", move |e| {
                println!("Received a state-changed event, re-emitting");
                handle.emit("state-changed", e.payload()).unwrap();
            });

            let mut reminder = TaskReminder {
                task_definitions: std::vec::Vec::new(),
                task_instances: std::collections::BinaryHeap::new(),
                calculated_instances: 0,
            };

            reminder.load_task_definitions();

            app.manage(Mutex::new(reminder));

            let handle = app.handle().clone();
            let mut window_counter = 0;
            thread::spawn(move || loop {
                let sleep_duration = {
                    let state = handle.state::<Mutex<TaskReminder>>();
                    let mut reminder = state.lock().unwrap();

                    let should_remind = if let Some(next) = reminder.task_instances.peek() {
                        next.0.timestamp <= Utc::now()
                    } else {
                        false
                    };

                    if should_remind {
                        if let Some(mut next) = reminder.task_instances.peek_mut() {
                            if !next.0.window_spawned {
                                println!("Reminding task!!!: {:?}", next);
                                window_counter += 1;

                                spawn_reminder(
                                    handle.clone(),
                                    next.0.name.clone(),
                                    window_counter.to_string(),
                                );

                                next.0.window_spawned = true;
                            }
                        }
                        Duration::from_millis(100)
                    } else if let Some(next) = reminder.task_instances.peek() {
                        let duration = (next.0.timestamp - Utc::now()).to_std().unwrap();
                        std::cmp::min(duration, Duration::from_secs(60))
                    } else {
                        Duration::from_secs(1)
                    }
                };
                std::thread::sleep(sleep_duration);
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            create_task,
            maximize,
            minimize,
            close,
            // update_task,
            // delete_task,
            get_next_task,
            complete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
