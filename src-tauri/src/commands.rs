use std::sync::Mutex;

use chrono::{DateTime, Utc};
use tauri::State;
use uuid::Uuid;

use crate::{
    config::{self, Settings},
    tasks::{Recurrence, TaskDefinition, TaskInstance, TaskReminder},
};

#[tauri::command]
pub fn get_task_definition(
    state: State<'_, Mutex<TaskReminder>>,
    id: Uuid,
) -> Result<Option<TaskDefinition>, String> {
    let reminder = state.lock().map_err(|e| e.to_string())?;

    if let Some(reminder) = reminder.get_task_definition(id) {
        Ok(Some(reminder.clone()))
    } else {
        Ok(None)
    }
}

// TODO: Add paging here for listing in the GUI
#[tauri::command]
pub fn get_tasks(state: State<'_, Mutex<TaskReminder>>) -> Result<Vec<TaskInstance>, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let tasks = state.get_tasks(0);
    // println!("GET TASKS: {:?}", tasks);

    Ok(tasks)
}

#[tauri::command]
pub fn get_next_task(state: State<'_, Mutex<TaskReminder>>) -> Result<TaskInstance, String> {
    let state = state.lock().map_err(|e| e.to_string())?;

    if let Some(task) = state.get_next_task() {
        println!("Loaded next task for popup: {:?}", task);
        Ok(task.clone())
    } else {
        Err("No tasks available".to_string())
    }
}

#[tauri::command]
pub fn create_task(
    state: State<'_, Mutex<TaskReminder>>,
    name: String,
    desc: Option<String>,
    start: String,
    recurrence: Option<Recurrence>,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let task_reminder = &mut state;

    let parsed_start = start
        .parse::<DateTime<Utc>>()
        .map_err(|e| format!("Invalid start datetime: {}", e))?;

    let new_task = TaskDefinition {
        id: Uuid::new_v4(),
        name,
        desc,
        start: parsed_start,
        recurrence,
    };

    task_reminder
        .create_task_definition(new_task)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_task(
    state: State<'_, Mutex<TaskReminder>>,
    task: TaskDefinition,
) -> Result<(), String> {
    let mut task_reminder = state.lock().map_err(|e| e.to_string())?;

    let new_definition = TaskDefinition {
        id: task.id,
        name: task.name,
        desc: task.desc,
        start: task.start,
        recurrence: task.recurrence,
    };

    task_reminder
        .update_task_definition(new_definition)
        .map_err(|e| e.to_string())?;

    Ok(())
    //  else {
    //     Err("Task definition not found".to_string())
    // }
}

#[tauri::command]
pub fn delete_task(state: State<'_, Mutex<TaskReminder>>, id: Uuid) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let task_reminder = &mut state;

    task_reminder
        .delete_task_definition(id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn complete_task(
    state: State<'_, Mutex<TaskReminder>>,
    task: TaskInstance,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let task_reminder = &mut state;

    task_reminder
        .mark_task_completed(task)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    let settings = config::get().settings.clone();

    Ok(settings)
}

#[tauri::command]
pub fn update_settings(settings: Settings) -> Result<(), String> {
    println!("Settings saved {:?}", settings);

    config::update_settings(settings).map_err(|e| e.to_string())?;

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
    window.hide().unwrap();
    Ok(())
}
