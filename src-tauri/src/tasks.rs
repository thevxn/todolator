use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;
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
    pub fn push(&mut self, task: Task) {
        self.tasks.push(Reverse(task));
        self.save_tasks().unwrap();
    }

    fn save_tasks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tasks: Vec<Task> = self.tasks.iter().map(|t| t.0.clone()).collect();
        let json_string = serde_json::to_string(&tasks).unwrap();
        fs::write("./tasks.json", json_string).unwrap();

        Ok(())
    }

    pub fn load_tasks(&mut self) {
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

    pub fn get_tasks(&mut self) -> Vec<Task> {
        let mut tasks: Vec<Task> = self.tasks.iter().map(|t| t.0.clone()).collect();
        tasks.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        tasks
    }

    pub fn get_next_task() {
        // TODO
    }

    pub fn update_task(&mut self, task: Task) {
        let mut v = self.tasks.clone().into_vec();
        if let Some(t) = v.iter_mut().find(|el| el.0.id == task.id) {
            t.0.name = task.name;
        };

        self.tasks = v.into();
        self.save_tasks().unwrap();
    }

    pub fn delete_task(&mut self, id: Uuid) {
        let mut v = self.tasks.clone().into_vec();
        if let Some(t) = v.iter_mut().position(|el| el.0.id == id) {
            v.remove(t);
        };

        self.tasks = v.into();
        self.save_tasks().unwrap();
    }
}
