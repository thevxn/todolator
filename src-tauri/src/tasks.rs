use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
enum RECURRENCE {
    // TODO: Take into acount calendar days/months/leap years/...
    NONE,
    HOURLY = 1,
    DAILY = 24,
    WEEKLY = 24 * 7,
    MONTHLY = 24 * 30,
    YEARLY = 24 * 30 * 12,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskDefinition {
    pub id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub first_recurrence: DateTime<Utc>,
    pub recurrence_type: RECURRENCE,
}

#[derive(Debug, Clone, PartialOrd, Eq)]
pub struct TaskInstance {
    pub definition_id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub reminded: bool,
}

impl PartialEq for TaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Ord for TaskInstance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct TaskExt {
//     pub name: String,
//     pub desc: Option<String>,
//     pub timestamp: DateTime<Utc>,
// }

static PAGE_SIZE: i32 = 30;
pub struct TaskReminder {
    pub task_definitions: Vec<TaskDefinition>,
    pub task_instances: BinaryHeap<std::cmp::Reverse<TaskInstance>>,
}

impl TaskReminder {
    pub fn push_task_definition(&mut self, definition: TaskDefinition) {
        self.task_definitions.push(definition);
    }

    pub fn push_task_instance(&mut self, instance: TaskInstance) {
        self.task_instances.push(Reverse(instance));
    }

    fn save_task_definitions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tasks: Vec<TaskDefinition> = self.task_definitions.iter().map(|t| t.clone()).collect();
        let json_string = serde_json::to_string(&tasks)?;
        fs::write("./tasks.json", json_string)?;

        Ok(())
    }

    pub fn load_task_definitions(&mut self) {
        let path = "tasks.json";
        if !Path::new(path).exists() {
            fs::write(path, "[]").expect("Failed to create tasks file")
        }

        let data = fs::read_to_string("./tasks.json").unwrap();
        let parsed = serde_json::from_str::<Vec<TaskDefinition>>(&data).unwrap();

        // TODO: Create a new method to iterate over the task definitions and calculate the first 30(?) recurrences (instances) for each one to hold in memory
        parsed.iter().for_each(|t| {
            self.push_task_definition(t.clone());
        });

        println!("loaded tasks {:?}", self.task_instances)
    }

    /// Calculates task instances from definitions on demand.
    pub fn get_task_instances(self, page: i32) -> Vec<TaskInstance> {
        let definitions = self.task_definitions;
        let mut instances = Vec::new();

        // For each definition:
        // Create 30(?) instances, where each instance has a timestamp of first start + current recurrence number offset
        definitions.iter().for_each(|d| {
            // Calculate the first 30 recurrences
            for i in 0..29 {
                let interval_hours = match d.recurrence_type {
                    RECURRENCE::NONE => 0,
                    _ => d.recurrence_type as i64,
                };

                instances.push(TaskInstance {
                    definition_id: d.id,
                    name: d.name.clone(),
                    desc: d.desc.clone(),
                    timestamp: d.first_recurrence + Duration::hours(i * interval_hours),
                    reminded: false,
                });

                match d.recurrence_type {
                    RECURRENCE::NONE => break,
                    _ => continue,
                }
            }
        });

        let item_from = (page * PAGE_SIZE) as usize;
        let item_to = item_from + PAGE_SIZE as usize;

        instances[item_from..item_to].to_vec()
    }

    // TODO: Is this needed to be kept?
    // pub fn get_tasks(&mut self) -> Vec<TaskExt> {
    //     let mut tasks: Vec<TaskDefinition> =
    //         self.task_instances.iter().map(|t| t.0.clone()).collect();
    //     tasks.sort();

    //     tasks.into_iter().map(TaskExt::from).collect()
    // }

    pub fn get_next_task(&mut self) -> Option<TaskInstance> {
        match self.task_instances.peek() {
            Some(reverse_task) => Some(reverse_task.0.clone().into()),
            None => None,
        }
    }

    pub fn mark_task_completed(&mut self) {
        self.task_instances.pop();
        println!(
            "Task marked as completed. Remaining tasks: {:?}",
            self.task_instances
        )
    }

    pub fn update_task(&mut self, definition: TaskDefinition) {
        let mut definitions = self.task_definitions.clone();
        if let Some(task) = definitions.iter_mut().find(|d| d.id == definition.id) {
            // TODO: Why am I only updating the name here??
            task.name = definition.name;
        };

        // TODO: If updating all recurrences, just update the underlying definition and recalculate the task instance list.
        // If updating a specific recurrence or from a specific recurrence onward, split the current definition into two.
        // self.task_instances = definitions.into();
        // self.save_task_definitions().unwrap();
    }

    pub fn delete_task(&mut self, id: Uuid) {
        let mut definitions = self.task_definitions.clone();
        if let Some(t) = definitions.iter_mut().position(|d| d.id == id) {
            definitions.remove(t);
        };

        self.task_definitions = definitions;
        self.save_task_definitions().unwrap();
    }
}
