use std::sync::Mutex;

use chrono::{DateTime, Utc};
use tauri::State;
use uuid::Uuid;

use crate::tasks::{Recurrence, TaskDefinition, TaskInstance, TaskReminder};

// TODO: Add paging here for listing in the GUI
#[tauri::command]
pub fn get_tasks(state: State<'_, Mutex<TaskReminder>>) -> Result<Vec<TaskInstance>, String> {
    let mut state = state.lock().unwrap();

    let tasks = state.get_tasks(0);
    // println!("GET TASKS: {:?}", tasks);

    Ok(tasks)
}

#[tauri::command]
pub fn get_next_task(state: State<'_, Mutex<TaskReminder>>) -> Result<TaskInstance, String> {
    let state = state.lock().unwrap();

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
    recurrence_minutes: Option<i64>,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    let new_task = TaskDefinition {
        id: Uuid::new_v4(),
        name,
        desc,
        start: start.parse::<DateTime<Utc>>().unwrap(),
        recurrence: match recurrence_minutes {
            Some(minutes) => Some(Recurrence::Recurring {
                last_recurrence: Some(start.parse::<DateTime<Utc>>().unwrap()),
                minutes,
                exceptions: None,
            }),
            None => None,
        },
    };

    task_reminder.create_task_definition(new_task).unwrap();

    Ok(())
}

#[tauri::command]
pub fn update_task(
    state: State<'_, Mutex<TaskReminder>>,
    task: TaskInstance,
) -> Result<(), String> {
    let mut task_reminder = state.lock().unwrap();

    if let Some(definition) = task_reminder
        .task_definitions
        .iter()
        .find(|d| d.id == task.definition_id)
    {
        let new_definition = TaskDefinition {
            id: task.definition_id,
            name: task.name,
            desc: task.desc,
            start: task.timestamp,
            recurrence: definition.recurrence.clone(),
        };

        task_reminder.update_task_definition(new_definition);
        Ok(())
    } else {
        Err("Task definition not found".to_string())
    }
}

#[tauri::command]
pub fn delete_task(state: State<'_, Mutex<TaskReminder>>, id: Uuid) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    task_reminder.delete_task_definition(id);

    Ok(())
}

#[tauri::command]
pub fn complete_task(
    state: State<'_, Mutex<TaskReminder>>,
    task: TaskInstance,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let task_reminder = &mut state;

    task_reminder.mark_task_completed(task);

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
