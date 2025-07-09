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
    App, AppHandle, Manager,
};

use crate::{
    commands::{
        close, create_task, delete_task, get_next_task, get_tasks, maximize, minimize, update_task,
    },
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
        .inner_size(500.00, 250.00)
        .build()
        .unwrap();
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            setup_tray(app);
            // Spawn a new window for a pop-up reminder
            let handle = app.handle().clone();

            let mut reminder = TaskReminder {
                tasks: std::collections::BinaryHeap::new(),
            };

            reminder.load_tasks();

            app.manage(Mutex::new(reminder));

            let mut window_counter = 0;
            thread::spawn(move || loop {
                let sleep_duration = {
                    let state = handle.state::<Mutex<TaskReminder>>();
                    let mut reminder = state.lock().unwrap();

                    let should_remind = if let Some(next) = reminder.tasks.peek() {
                        next.0.timestamp <= Utc::now()
                    } else {
                        false
                    };

                    if should_remind {
                        if let Some(mut next) = reminder.tasks.peek_mut() {
                            if !next.0.reminded {
                                println!("Reminding task!!!: {:?}", next);
                                window_counter += 1;

                                spawn_reminder(
                                    handle.clone(),
                                    next.0.name.clone(),
                                    window_counter.to_string(),
                                );

                                next.0.reminded = true;
                            }

                            // TODO:
                            // This does not remove the reminder from the JSON file. It will be needed to split this into 2 separate data sets ig.
                            // 1 for the current reminder heap and 1 for the complete list of all tasks (including the already reminded ones),
                            // unless the reminded ones should just be deleted, which is also an option..
                            // reminder.tasks.pop();
                        }
                        Duration::from_millis(100)
                    } else if let Some(next) = reminder.tasks.peek() {
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
            update_task,
            delete_task,
            get_next_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
