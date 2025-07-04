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
