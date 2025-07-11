use std::sync::Mutex;

use chrono::{DateTime, Utc};
use tauri::State;
use uuid::Uuid;

use crate::tasks::{self, TaskExt, TaskReminder};

#[tauri::command]
pub fn get_tasks(state: State<'_, Mutex<TaskReminder>>) -> Result<Vec<tasks::TaskExt>, String> {
    let mut state = state.lock().unwrap();

    let tasks = state.get_tasks();
    println!("GET TASKS: {:?}", tasks);

    Ok(tasks)
}

#[tauri::command]
pub fn get_next_task(state: State<'_, Mutex<TaskReminder>>) -> Result<TaskExt, String> {
    let mut state = state.lock().unwrap();

    if let Some(task) = state.get_next_task() {
        println!("NEXT TASK: {:?}", task);
        Ok(task)
    } else {
        Err("No tasks available".to_string())
    }
}

#[tauri::command]
pub fn create_task(
    state: State<'_, Mutex<TaskReminder>>,
    name: String,
    desc: Option<String>,
    timestamp: DateTime<Utc>,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    let new_task = tasks::Task {
        id: Uuid::new_v4(),
        name,
        desc,
        timestamp,
        reminded: false,
    };

    task_reminder.push(new_task);

    Ok(())
}

#[tauri::command]
pub fn update_task(state: State<'_, Mutex<TaskReminder>>, task: TaskExt) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    task_reminder.update_task(task);

    Ok(())
}

#[tauri::command]
pub fn delete_task(state: State<'_, Mutex<TaskReminder>>, id: Uuid) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    task_reminder.delete_task(id);

    Ok(())
}

#[tauri::command]
pub fn complete_task(state: State<'_, Mutex<TaskReminder>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    task_reminder.mark_task_completed();

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
