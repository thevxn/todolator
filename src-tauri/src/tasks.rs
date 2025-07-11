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
    pub reminded: bool,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskExt {
    pub name: String,
    pub desc: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub id: Uuid,
}

impl From<Task> for TaskExt {
    fn from(task: Task) -> Self {
        TaskExt {
            name: task.name,
            desc: task.desc,
            timestamp: task.timestamp,
            id: task.id,
        }
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
        let tasks: Vec<TaskExt> = self
            .tasks
            .iter()
            .map(|t| TaskExt::from(t.0.clone()))
            .collect();
        let json_string = serde_json::to_string(&tasks)?;
        fs::write("./tasks.json", json_string)?;

        Ok(())
    }

    pub fn load_tasks(&mut self) {
        let path = "tasks.json";
        if !Path::new(path).exists() {
            fs::write(path, "[]").expect("Failed to create tasks file")
        }

        let data = fs::read_to_string("./tasks.json").unwrap();
        let parsed = serde_json::from_str::<Vec<TaskExt>>(&data).unwrap();

        parsed.iter().for_each(|t| {
            self.push(Task {
                id: t.id,
                name: t.name.clone(),
                desc: t.desc.clone(),
                timestamp: t.timestamp,
                reminded: false,
            });
        });

        println!("loaded tasks {:?}", self.tasks)
    }

    pub fn get_tasks(&mut self) -> Vec<TaskExt> {
        let mut tasks: Vec<Task> = self.tasks.iter().map(|t| t.0.clone()).collect();
        tasks.sort();

        tasks.into_iter().map(TaskExt::from).collect()
    }

    pub fn get_next_task(&mut self) -> Option<TaskExt> {
        match self.tasks.peek() {
            Some(reverse_task) => Some(reverse_task.0.clone().into()),
            None => None,
        }
    }

    pub fn mark_task_completed(&mut self) {
        self.tasks.pop();
        println!(
            "Task marked as completed. Remaining tasks: {:?}",
            self.tasks
        )
    }

    pub fn update_task(&mut self, task: TaskExt) {
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
