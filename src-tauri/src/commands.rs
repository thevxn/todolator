use std::fs;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::tasks;

#[tauri::command]
pub fn get_tasks() -> Result<Vec<tasks::Task>, String> {
    let tasks = tasks::TASKS.lock().map_err(|e| e.to_string())?;
    Ok(tasks.clone())
}

#[tauri::command]
pub fn create_task(
    name: String,
    desc: Option<String>,
    timestamp: DateTime<Utc>,
) -> Result<(), String> {
    let mut tasks = tasks::TASKS.lock().map_err(|e| e.to_string())?;

    let new_task = tasks::Task {
        id: Uuid::new_v4(),
        name: name.to_string(),
        desc: match desc {
            Some(d) => Some(d.to_string()),
            None => None,
        },
        timestamp: timestamp,
    };

    tasks.push(new_task);

    let json_string = serde_json::to_string(&*tasks).map_err(|e| e.to_string())?;
    fs::write("./tasks.json", json_string).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_task(
    id: Uuid,
    name: String,
    desc: Option<String>,
    timestamp: DateTime<Utc>,
) -> Result<(), String> {
    let mut tasks = tasks::TASKS.lock().map_err(|e| e.to_string())?;

    let task = tasks.iter_mut().find(|el| el.id == id);

    if let Some(t) = task {
        t.name = name;
        if let Some(d) = desc {
            t.desc = Some(d);
        };
        t.timestamp = timestamp;
    }

    // TODO: Refactor this into its own fn (in all places where its used)
    let json_string = serde_json::to_string(&*tasks).map_err(|e| e.to_string())?;
    fs::write("./tasks.json", json_string).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn maximize(window: tauri::Window) -> Result<(), String> {
    if !window.is_maximized().unwrap() {
        window.maximize().unwrap();
        return Ok(());
    }
    window.unmaximize().unwrap();
    Ok(())
}

#[tauri::command]
pub fn minimize(window: tauri::Window) -> Result<(), String> {
    if !window.is_minimized().unwrap() {
        window.minimize().unwrap();
        return Ok(());
    }
    window.unminimize().unwrap();
    Ok(())
}

#[tauri::command]
pub fn close(window: tauri::Window) -> Result<(), String> {
    window.close().unwrap();
    Ok(())
}
