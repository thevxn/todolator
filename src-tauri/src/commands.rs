use chrono::{DateTime, Utc};

use crate::tasks;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
        name: name.to_string(),
        desc: match desc {
            Some(d) => Some(d.to_string()),
            None => None,
        },
        timestamp: timestamp,
    };

    tasks.push(new_task);
    Ok(())
}
