use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub desc: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub id: Uuid,
}

// TODO: Probably not needed to keep all tasks in memory at all times
pub static TASKS: Lazy<Mutex<Vec<Task>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn load_tasks() {
    let path = "tasks.json";
    if !Path::new(path).exists() {
        fs::write(path, "[]").expect("Failed to create tasks file")
    }

    let data = fs::read_to_string("./tasks.json").unwrap();
    let parsed = serde_json::from_str::<Vec<Task>>(&data).unwrap();

    let mut tasks = TASKS.lock().unwrap();
    *tasks = parsed;

    println!("{:?}", *tasks)
}
