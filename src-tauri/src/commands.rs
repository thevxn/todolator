use std::sync::Mutex;

use chrono::{DateTime, Utc};
// use chrono::{DateTime, Utc};
use tauri::State;
use uuid::Uuid;
// use uuid::Uuid;

use crate::tasks::{TaskDefinition, TaskInstance, TaskReminder};

// TODO: Add paging here for listing in the GUI
#[tauri::command]
pub fn get_tasks(state: State<'_, Mutex<TaskReminder>>) -> Result<Vec<TaskInstance>, String> {
    let mut state = state.lock().unwrap();

    let tasks = state.get_tasks(0);
    println!("GET TASKS: {:?}", tasks);

    Ok(tasks)
}

#[tauri::command]
pub fn get_next_task(state: State<'_, Mutex<TaskReminder>>) -> Result<TaskInstance, String> {
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
    recurrence_minutes: Option<i64>,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    let new_task = TaskDefinition {
        id: Uuid::new_v4(),
        name,
        desc,
        first_recurrence: timestamp,
        recurrence_minutes,
    };

    task_reminder.create_task_definition(new_task);

    Ok(())
}

// #[tauri::command]
// pub fn update_task(state: State<'_, Mutex<TaskReminder>>, task: TaskExt) -> Result<(), String> {
//     let mut state = state.lock().unwrap();
//     let task_reminder = &mut state;

//     task_reminder.update_task(task);

//     Ok(())
// }

// #[tauri::command]
// pub fn delete_task(state: State<'_, Mutex<TaskReminder>>, id: Uuid) -> Result<(), String> {
//     let mut state = state.lock().unwrap();
//     let task_reminder = &mut state;

//     task_reminder.delete_task(id);

//     Ok(())
// }

// TODO: Provide the Definition ID from the GUI
#[tauri::command]
pub fn complete_task(
    state: State<'_, Mutex<TaskReminder>>,
    definition_id: Uuid,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    task_reminder.mark_task_completed(definition_id);

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
