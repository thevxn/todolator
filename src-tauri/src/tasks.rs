use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Recurrence {
    None,
    Recurring {
        last_recurrence: DateTime<Utc>,
        minutes: i64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskDefinition {
    pub id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
    pub start: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialOrd, Eq, Serialize, Deserialize)]
pub struct TaskInstance {
    pub definition_id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub window_spawned: bool,
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

static PAGE_SIZE: i32 = 30;
pub struct TaskReminder {
    pub task_definitions: Vec<TaskDefinition>,
    pub task_instances: BinaryHeap<std::cmp::Reverse<TaskInstance>>,
    pub calculated_instances: usize,
}

impl TaskReminder {
    fn push_task_definition(&mut self, definition: TaskDefinition) {
        self.task_definitions.push(definition);
    }

    pub fn create_task_definition(&mut self, definition: TaskDefinition) {
        self.push_task_definition(definition);
        self.save_task_definitions().unwrap();
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

        parsed.iter().for_each(|t| {
            self.push_task_definition(t.clone());
        });

        // self.save_task_definitions().unwrap();
        // self.generate_task_instances(0);

        println!("Loaded task definitions: {:?}", self.task_definitions)
    }

    fn get_task_definition(&self, id: Uuid) -> Option<TaskDefinition> {
        let definitions = &self.task_definitions;
        if let Some(def) = definitions.iter().find(|d| d.id == id) {
            Some(def.clone())
        } else {
            None
        }

        // self.task_definitions = definitions;
        // self.save_task_definitions().unwrap();
        // self.generate_task_instances(0);
    }

    /// Calculates task instances from definitions on demand.
    pub fn generate_task_instances(&mut self, page: i32) {
        let definitions = self.task_definitions.clone();

        // Iterate over definitions - generate one instance for each definition on each iteration (if recurring - if not, only the first iteration generates an instance). Break once the total generated instances are at (page + 1) * PAGE_SIZE.
        //
        // These generated instances are held in memory, so with each page the size of the tasks stored in memory grows.
        let instances_required = ((page + 1) * PAGE_SIZE) as i64;

        for i in 0..instances_required {
            definitions.iter().for_each(|d| {
                if i == 0 {
                    println!("Pushing first instance of a definition (recurrence does not matter)");
                    self.push_task_instance(TaskInstance {
                        definition_id: d.id,
                        name: d.name.clone(),
                        desc: d.desc.clone(),
                        timestamp: d.start,
                        window_spawned: false,
                    });
                } else {
                    if let Some(Recurrence::Recurring {
                        minutes,
                        last_recurrence,
                    }) = d.recurrence
                    {
                        println!("Pushing instance of a recurring definition");
                        self.push_task_instance(TaskInstance {
                            definition_id: d.id,
                            name: d.name.clone(),
                            desc: d.desc.clone(),
                            timestamp: last_recurrence + Duration::minutes(i * minutes),
                            window_spawned: false,
                        });
                    }
                    //  else {
                    //     println!("2nd+ iteration: non-recurring definition or not enough definitions to generate a full page, nothing will be pushed")
                    // }
                }
            });
        }

        // let item_from = (page * PAGE_SIZE) as usize;
        // let item_to = item_from + PAGE_SIZE as usize;
        println!("Calculated instances: {:?}", self.task_instances.len());
        self.calculated_instances = self.task_instances.len();
    }

    pub fn get_tasks(&mut self, page: i32) -> Vec<TaskInstance> {
        if (page + 1) * PAGE_SIZE > self.calculated_instances as i32 {
            // I think this is not necessary anymore at this point.
            // The latest definitions will always be loaded in memory and saved to JSON when creating/updating/deleting.
            // println!("Loading definitions from JSON file...");
            // self.task_definitions.clear();
            // self.load_task_definitions();
            println!("Generating task instances...");
            self.task_instances.clear();
            self.generate_task_instances(page);
        }

        let item_from = (page * PAGE_SIZE) as usize;
        let item_to = item_from + PAGE_SIZE as usize;
        let item_to = item_to.min(self.task_instances.len());

        if self.calculated_instances > 0 && item_from < self.task_instances.len() {
            let tasks: Vec<TaskInstance> =
                self.task_instances.iter().map(|t| t.0.clone()).collect();
            // tasks.sort();

            return tasks[item_from..item_to].to_vec();
        }
        std::vec::Vec::new()
    }

    // TODO: Once an instance is reminded, if its parent definition is not recurring, delete the definition along with the instance.
    pub fn get_next_task(&mut self) -> Option<TaskInstance> {
        match self.task_instances.peek() {
            Some(reverse_task) => Some(reverse_task.0.clone().into()),
            None => None,
        }
    }

    pub fn mark_task_completed(&mut self, definition_id: Uuid) {
        let instances = &mut self.task_instances;
        instances.pop();

        println!(
            "Task marked as completed. Remaining tasks: {:?}",
            self.task_instances
        );

        if let Some(definition) = self.get_task_definition(definition_id) {
            if let None = definition.recurrence {
                self.delete_task_definition(definition_id);
            }
        }
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

    pub fn delete_task_definition(&mut self, id: Uuid) {
        let definitions = &mut self.task_definitions;
        if let Some(t) = definitions.iter_mut().position(|d| d.id == id) {
            definitions.remove(t);
        };

        self.save_task_definitions().unwrap();
        // TODO: Is this needed here? Probably not?
        // self.generate_task_instances(0);
    }
}
