use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, Eq)]
pub struct Task {
    pub name: String,
    pub desc: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub id: Uuid,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

pub struct TaskReminder {
    pub tasks: BinaryHeap<std::cmp::Reverse<Task>>,
}

impl TaskReminder {
    fn push(&mut self, task: Task) {
        self.tasks.push(Reverse(task));
    }

    fn load(&mut self) {
        let path = "tasks.json";
        if !Path::new(path).exists() {
            fs::write(path, "[]").expect("Failed to create tasks file")
        }

        let data = fs::read_to_string("./tasks.json").unwrap();
        let parsed = serde_json::from_str::<Vec<Task>>(&data).unwrap();

        parsed.iter().for_each(|el| {
            self.push(el.clone());
        });

        println!("loaded tasks {:?}", self.tasks)
    }

    pub fn run(&mut self) {
        self.load();
        loop {
            if let Some(Reverse(next)) = self.tasks.peek() {
                let now = Utc::now();

                if next.timestamp <= now {
                    println!("Reminding task!!!: {:?}", next);
                    self.tasks.pop();
                } else {
                    let duration = (next.timestamp - now).to_std().unwrap();
                    std::thread::sleep(duration);
                }
            } else {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
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

    // println!("{:?}", *tasks)
}
