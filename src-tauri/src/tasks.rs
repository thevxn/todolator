use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Recurrence {
    None,
    Recurring {
        last_recurrence: Option<DateTime<Utc>>,
        minutes: i64,
        exceptions: Option<Vec<DateTime<Utc>>>,
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

    fn push_task_instance(&mut self, instance: TaskInstance) {
        self.task_instances.push(Reverse(instance));
    }

    fn get_task_definition_mut(&mut self, id: Uuid) -> Option<&mut TaskDefinition> {
        self.task_definitions.iter_mut().find(|d| d.id == id)
    }

    fn save_task_definitions(&self) -> Result<(), Box<dyn Error>> {
        let tasks: Vec<TaskDefinition> = self.task_definitions.iter().map(|t| t.clone()).collect();
        let json_string = serde_json::to_string(&tasks)?;
        fs::write("./tasks.json", json_string)?;

        Ok(())
    }

    pub fn create_task_definition(
        &mut self,
        definition: TaskDefinition,
    ) -> Result<(), Box<dyn Error>> {
        self.push_task_definition(definition);
        self.save_task_definitions()?;
        Ok(())
    }

    pub fn load_task_definitions(&mut self) -> Result<(), Box<dyn Error>> {
        let path = "tasks.json";
        if !Path::new(path).exists() {
            fs::write(path, "[]").expect("Failed to create tasks file")
        }

        let data = fs::read_to_string("./tasks.json")?;
        let parsed = serde_json::from_str::<Vec<TaskDefinition>>(&data)?;

        parsed.iter().for_each(|t| {
            self.push_task_definition(t.clone());
        });

        println!("Loaded task definitions: {:?}", self.task_definitions);

        Ok(())
    }

    /// Calculates task instances from definitions on demand.
    pub fn generate_task_instances(&mut self, page: i32) {
        println!("Beginning to recalculate new instances");
        let definitions = self.task_definitions.clone();

        // If a task already has a window spawned, it must be set accordingly on the new instance
        let mut window_spawned_map: HashMap<(Uuid, DateTime<Utc>), bool> = HashMap::new();
        self.task_instances.iter().for_each(|t| {
            window_spawned_map.insert((t.0.definition_id, t.0.timestamp), t.0.window_spawned);
        });

        self.task_instances.clear();

        // Iterate over definitions - generate one instance for each definition on the first iteration.
        //
        // If the definition is not recurring, no further instances are generated.
        // Otherwise, instances keep being generated on each iteration up to the size of `instances_required`, taking into account exceptions.
        //
        // Break once the total generated instances are at (page + 1) * PAGE_SIZE.
        //
        // These generated instances are held in memory, so with each page the size of the tasks stored in memory grows along with the number of instances required to construct the page.
        let instances_required = ((page + 1) * PAGE_SIZE) as i64;

        let mut instances: Vec<TaskInstance> = Vec::new();

        definitions.iter().for_each(|d| {
            for i in 0..instances_required {
                let recurrence_info = if let Some(Recurrence::Recurring {
                    last_recurrence,
                    minutes,
                    exceptions,
                }) = &d.recurrence
                {
                    Some((last_recurrence, minutes, exceptions))
                } else {
                    None
                };

                // If the task is not recurring and the first instance has already been spawned, break
                if recurrence_info.is_none() && i > 0 {
                    break;
                }

                let timestamp = match recurrence_info {
                    Some((last_recurrence, minutes, _exceptions)) => match *last_recurrence {
                        Some(last) => last + Duration::minutes((i + 1) * minutes),

                        None => d.start,
                    },
                    _ => d.start,
                };

                let exceptions = if let Some(recurrence) = recurrence_info {
                    recurrence.2.clone()
                } else {
                    None
                };

                let should_skip = match exceptions {
                    Some(list) => list.iter().any(|e| e == &timestamp),
                    None => false,
                };
                if !should_skip {
                    instances.push(TaskInstance {
                        definition_id: d.id,
                        name: d.name.clone(),
                        desc: d.desc.clone(),
                        timestamp,
                        window_spawned: false,
                    });
                }
            }
        });

        instances.sort_by_key(|i| i.timestamp);
        instances.iter_mut().take(PAGE_SIZE as usize).for_each(|i| {
            if window_spawned_map.contains_key(&(i.definition_id, i.timestamp)) {
                i.window_spawned = *window_spawned_map
                    .get(&(i.definition_id, i.timestamp))
                    .unwrap_or(&false);
            };
            self.push_task_instance(i.clone());
        });

        self.calculated_instances = self.task_instances.len();
        println!("Calculated instances: {:?}", self.task_instances.len());
        // println!("{:?}", self.task_instances)
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

    pub fn get_next_task(&self) -> Option<&TaskInstance> {
        self.task_instances.peek().map(|task| &task.0)
    }

    pub fn mark_task_completed(&mut self, task: TaskInstance) -> Result<(), Box<dyn Error>> {
        let instances = &mut self.task_instances;
        instances.pop();

        println!(
            "Task marked as completed (popped off the instances heap). Timestamp: {:?}",
            task.timestamp
        );

        if let Some(definition) = self.get_task_definition_mut(task.definition_id) {
            if let Some(Recurrence::Recurring {
                last_recurrence,
                minutes: _,
                exceptions: _,
            }) = &mut definition.recurrence
            {
                *last_recurrence = Some(task.timestamp);
                self.save_task_definitions()?;
            } else {
                self.delete_task_definition(task.definition_id)?;
            }
        }

        Ok(())
    }

    pub fn update_task_definition(&mut self, updated: TaskDefinition) {
        let definitions = &mut self.task_definitions;
        if let Some(task) = definitions.iter_mut().find(|d| d.id == updated.id) {
            *task = updated;

            print!("Updated task definition: {:?}", task)
        };

        // TODO: If updating all recurrences, just update the underlying definition and recalculate the task instance list.
        // If updating a specific recurrence or from a specific recurrence onward, split the current definition into two.
        // self.task_instances = definitions.into();
        // self.save_task_definitions().unwrap();
    }

    pub fn delete_task_definition(&mut self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let definitions = &mut self.task_definitions;
        if let Some(t) = definitions.iter_mut().position(|d| d.id == id) {
            definitions.remove(t);
        };

        self.save_task_definitions()?;
        self.generate_task_instances(0);

        Ok(())
    }
}
