use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    name: String,
    timestamp: DateTime<Utc>,
}

pub static TASKS: Lazy<Mutex<Vec<Task>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn load_tasks() {
    let data = fs::read_to_string("./tasks.json").unwrap();
    let parsed = serde_json::from_str::<Vec<Task>>(&data).unwrap();
    let mut tasks = TASKS.lock().unwrap();
    *tasks = parsed;

    println!("{:?}", *tasks)
}
