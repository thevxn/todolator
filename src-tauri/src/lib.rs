mod commands;
mod tasks;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    App, AppHandle, Manager,
};

use crate::{
    commands::{close, create_task, get_tasks, maximize, minimize},
    tasks::load_tasks,
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
            // let handle = app.handle().clone();
            // spawn_reminder(handle);
            load_tasks();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            create_task,
            maximize,
            minimize,
            close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
