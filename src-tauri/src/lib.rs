use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    App, AppHandle, Manager,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    timestamp: DateTime<Utc>,
}

#[tauri::command]
fn get_tasks() -> Result<Vec<Task>, String> {
    let data = fs::read_to_string("./tasks.json").map_err(|e| e.to_string())?;
    let tasks: Vec<Task> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    println!("{:?}", tasks);
    // Err(String::from("error"))
    Ok(tasks)
}

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

fn spawn_reminder(handle: AppHandle) {
    std::thread::spawn(move || {
        WebviewWindowBuilder::new(
            &handle,
            "reminder",
            tauri::WebviewUrl::App("reminder.html".into()),
        )
        .title("Todolator Reminder")
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
            spawn_reminder(handle);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
