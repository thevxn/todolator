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
        last_recurrence: Option<DateTime<Utc>>,
        minutes: i64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskDefinition {
    pub id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub recurrence: Option<Recurrence>,
    pub start: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
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

impl PartialOrd for TaskInstance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
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

    fn get_task_definition(&mut self, id: Uuid) -> Option<&mut TaskDefinition> {
        self.task_definitions.iter_mut().find(|d| d.id == id)
    }

    /// Calculates task instances from definitions on demand.
    pub fn generate_task_instances(&mut self, page: i32) {
        let definitions = self.task_definitions.clone();

        self.task_instances.clear();

        // Iterate over definitions - generate one instance for each definition on each iteration (if recurring - if not, only the first iteration generates an instance). Break once the total generated instances are at (page + 1) * PAGE_SIZE.
        //
        // These generated instances are held in memory, so with each page the size of the tasks stored in memory grows.
        let instances_required = ((page + 1) * PAGE_SIZE) as i64;

        let mut instances: Vec<TaskInstance> = Vec::new();

        definitions.iter().for_each(|d| {
            for i in 0..instances_required {
                let timestamp = match d.recurrence {
                    Some(Recurrence::Recurring {
                        last_recurrence,
                        minutes,
                    }) => match last_recurrence {
                        Some(last) => last + Duration::minutes((i + 1) * minutes),

                        None => d.start,
                    },
                    _ => d.start,
                };

                instances.push(TaskInstance {
                    definition_id: d.id,
                    name: d.name.clone(),
                    desc: d.desc.clone(),
                    timestamp,
                    window_spawned: false,
                });

                // if i == 0 {
                //     println!("Pushing first instance of a definition (recurrence does not matter)");
                //     instances.push(TaskInstance {
                //         definition_id: d.id,
                //         name: d.name.clone(),
                //         desc: d.desc.clone(),
                //         // d.start if non-recurring
                //         // last_recurrence (if present, else fill 0) + minutes offset if recurring
                //         timestamp: if let Some(Recurrence::Recurring {
                //             last_recurrence,
                //             minutes,
                //         }) = d.recurrence
                //         {
                //             if last_recurrence.is_some() {
                //                 last_recurrence.unwrap() + Duration::minutes(i * minutes)
                //             } else {
                //                 d.start
                //             }
                //         } else {
                //             d.start
                //         },
                //         window_spawned: false,
                //     });
                // } else {
                //     if let Some(Recurrence::Recurring {
                //         minutes,
                //         last_recurrence,
                //     }) = d.recurrence
                //     {
                //         println!("Pushing instance of a recurring definition");
                //         instances.push(TaskInstance {
                //             definition_id: d.id,
                //             name: d.name.clone(),
                //             desc: d.desc.clone(),
                //             timestamp: Result(last_recurrence) + Duration::minutes(i * minutes),
                //             window_spawned: false,
                //         });
                //     } else {
                //         println!("Non-recurring instance, breaking");
                //         break;
                //     }
                // }
            }
        });

        instances.sort_by_key(|i| i.timestamp);
        instances
            .iter()
            .take(PAGE_SIZE as usize)
            .for_each(|i| self.push_task_instance(i.clone()));

        // self.task_instances = BinaryHeap::from(
        //     instances
        //         .into_iter()
        //         .take(instances_required as usize)
        //         .map(Reverse)
        //         .collect::<Vec<_>>(),
        // );

        self.calculated_instances = self.task_instances.len();
        println!("Calculated instances: {:?}", self.task_instances.len());
    }

    pub fn get_tasks(&mut self, page: i32) -> Vec<TaskInstance> {
        println!("Generating task instances...");
        self.generate_task_instances(page);

        let item_from = (page * PAGE_SIZE) as usize;
        let item_to = item_from + PAGE_SIZE as usize;
        let item_to = item_to.min(self.task_instances.len());

        if self.calculated_instances > 0 && item_from < self.task_instances.len() {
            let mut tasks: Vec<TaskInstance> =
                self.task_instances.iter().map(|t| t.0.clone()).collect();

            tasks.sort_by_key(|i| i.timestamp);

            return tasks[item_from..item_to].to_vec();
        }
        std::vec::Vec::new()
    }

    pub fn get_next_task(&mut self) -> Option<TaskInstance> {
        self.task_instances
            .peek()
            .map(|reverse_task| reverse_task.0.clone())
    }

    pub fn mark_task_completed(&mut self, task: TaskInstance) {
        let instances = &mut self.task_instances;
        instances.pop();

        println!(
            "Task marked as completed. Remaining tasks: {:?}",
            self.task_instances
        );

        if let Some(definition) = self.get_task_definition(task.definition_id) {
            if let Some(Recurrence::Recurring {
                last_recurrence,
                minutes: _,
            }) = &mut definition.recurrence
            {
                *last_recurrence = Some(task.timestamp);
                self.save_task_definitions().unwrap();
            } else {
                self.delete_task_definition(task.definition_id);
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
        self.generate_task_instances(0);
    }
}
