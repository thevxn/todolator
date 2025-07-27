mod commands;
mod config;
mod tasks;

use std::{
    error::Error,
    sync::Mutex,
    thread::{self},
    time::Duration,
};

use chrono::Utc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    App, AppHandle, Manager, UserAttentionType,
};

use crate::{
    commands::{
        close, complete_task, create_task, delete_task, get_next_task, get_settings,
        get_task_definition, get_tasks, maximize, minimize, update_settings, update_task,
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
                #[cfg(debug_assertions)]
                println!("quit menu item was clicked");

                app.exit(0);
            }
            _ => {
                #[cfg(debug_assertions)]
                println!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                #[cfg(debug_assertions)]
                println!("double left-click pressed");

                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            _ => {
                #[cfg(debug_assertions)]
                println!("unhandled event {event:?}");
            }
        })
        .build(app)
        .unwrap();
}

fn spawn_reminder(handle: AppHandle, task_name: String, label: &str) -> Result<(), Box<dyn Error>> {
    #[cfg_attr(not(target_os = "windows"), allow(unused_mut))]
    let mut builder = WebviewWindowBuilder::new(
        &handle,
        label,
        tauri::WebviewUrl::App("reminder.html".into()),
    )
    .title(task_name)
    .decorations(false)
    .resizable(true)
    .center()
    .focused(true)
    .always_on_top(true)
    .visible_on_all_workspaces(true)
    .inner_size(640.00, 350.00);

    #[cfg(target_os = "windows")]
    {
        builder = builder.drag_and_drop(true);
    }

    builder.build().unwrap();

    let window = handle.get_webview_window(&label.to_string());
    window
        .unwrap()
        .request_user_attention(Some(UserAttentionType::Critical))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            config::init(
                // path::Path::new("tasks.json").to_path_buf(),
                app.handle()
                    .clone()
                    .path()
                    .app_data_dir()
                    .expect("app data directory exists"),
            );

            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle()
                    .plugin(tauri_plugin_autostart::init(
                        MacosLauncher::LaunchAgent,
                        Some(vec!["--flag1", "--flag2"]),
                    ))
                    .unwrap();

                // Get the autostart manager
                let autostart_manager = app.autolaunch();

                let config = config::get();
                let settings = &config.settings;

                if autostart_manager.is_enabled().unwrap_or(false) {
                    if settings.autostart {
                        autostart_manager.enable().unwrap();
                    } else {
                        autostart_manager.disable().unwrap();
                    }
                }

                #[cfg(debug_assertions)]
                println!(
                    "registered for autostart? {}",
                    autostart_manager.is_enabled().unwrap()
                );
            }

            setup_tray(app);

            let mut reminder = TaskReminder {
                task_definitions: std::vec::Vec::new(),
                task_instances: std::collections::BinaryHeap::new(),
                calculated_instances: 0,
            };

            reminder.load_task_definitions()?;

            app.manage(Mutex::new(reminder));

            let mut window_counter = 0;
            let handle = app.handle().clone();
            thread::spawn(move || loop {
                let sleep_duration = {
                    let state = handle.state::<Mutex<TaskReminder>>();
                    let mut reminder = state.lock().unwrap();

                    let task = reminder.task_instances.peek_mut();

                    let should_remind = if let Some(next) = &task {
                        next.0.timestamp <= Utc::now()
                    } else {
                        false
                    };

                    if should_remind {
                        if let Some(mut next) = task {
                            if !next.0.window_spawned {
                                #[cfg(debug_assertions)]
                                println!("Reminding task!!!: {:?}", next.0.timestamp);
                                
                                window_counter += 1;

                                next.0.window_spawned = true;
                                spawn_reminder(
                                    handle.clone(),
                                    next.0.name.clone(),
                                    &window_counter.to_string(),
                                )
                                // TODO: Ensure better error handling
                                .unwrap();
                            }
                        }
                        Duration::from_millis(100)
                    } else if let Some(next) = task {
                        let duration = (next.0.timestamp - Utc::now()).to_std().unwrap();
                        std::cmp::min(duration, Duration::from_secs(5))
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
            get_next_task,
            complete_task,
            get_task_definition,
            get_settings,
            update_settings
        ]);

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let main_window = app.get_webview_window("main").expect("no main window");

            if main_window.is_minimized().unwrap_or(false) {
                main_window.unminimize().unwrap();
            } else {
                main_window.show().unwrap();
            }
        }));
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
