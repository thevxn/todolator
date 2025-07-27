use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs;
use uuid::Uuid;

use crate::config;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
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
        #[cfg(debug_assertions)]
        println!("Pushing def: {:?}", definition);

        self.task_definitions.push(definition)
    }

    fn push_task_instance(&mut self, instance: TaskInstance) {
        self.task_instances.push(Reverse(instance))
    }

    fn get_task_definition_mut(&mut self, id: Uuid) -> Option<&mut TaskDefinition> {
        self.task_definitions.iter_mut().find(|d| d.id == id)
    }

    fn save_task_definitions(&self) -> Result<(), Box<dyn Error>> {
        let tasks: Vec<TaskDefinition> = self.task_definitions.iter().map(|t| t.clone()).collect();
        let json_string = serde_json::to_string(&tasks)?;

        fs::write(&config::get().data_path, json_string)?;

        Ok(())
    }

    pub fn get_task_definition(&self, id: Uuid) -> Option<&TaskDefinition> {
        self.task_definitions.iter().find(|d| d.id == id)
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
        let data = fs::read_to_string(&config::get().data_path)?;
        let parsed = serde_json::from_str::<Vec<TaskDefinition>>(&data)?;

        parsed.iter().for_each(|t| {
            self.push_task_definition(t.clone());
        });

        #[cfg(debug_assertions)]
        println!("Loaded task definitions: {:?}", self.task_definitions);

        Ok(())
    }

    /// Calculates task instances from definitions on demand.
    /// Generates enough instances to provide the requested page.
    ///
    /// E.g., for page 0 with `PAGE_SIZE=30`, up to 30 instances per definition are generated. For page 2 with the same `PAGE_SIZE`, up to 90 instances per definition are generated.
    pub fn generate_task_instances(&mut self, page: i32) {
        #[cfg(debug_assertions)]
        println!("Beginning to recalculate new instances");

        let definitions = &self.task_definitions;

        // If any tasks already have a window spawned, the state must be kept and set accordingly on the new set of instances
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

                        None => d.start + Duration::minutes(i * minutes),
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
        instances
            .iter_mut()
            .take(instances_required as usize)
            .for_each(|i| {
                if window_spawned_map.contains_key(&(i.definition_id, i.timestamp)) {
                    i.window_spawned = *window_spawned_map
                        .get(&(i.definition_id, i.timestamp))
                        .unwrap_or(&false);
                };
                self.push_task_instance(i.clone());
            });

        self.calculated_instances = self.task_instances.len();

        #[cfg(debug_assertions)]
        println!("Calculated instances: {:?}", self.task_instances.len());
    }

    pub fn get_tasks(&mut self, page: i32) -> Vec<TaskInstance> {
        #[cfg(debug_assertions)]
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

        #[cfg(debug_assertions)]
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

    pub fn update_task_definition(
        &mut self,
        updated: TaskDefinition,
    ) -> Result<(), Box<dyn Error>> {
        let definitions = &mut self.task_definitions;
        if let Some(task) = definitions.iter_mut().find(|d| d.id == updated.id) {
            *task = updated;

            #[cfg(debug_assertions)]
            println!("Updated task definition: {:?}", task)
        };

        self.save_task_definitions()?;

        Ok(())

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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use chrono::TimeZone;
    use tempfile::tempdir;

    fn sample_task_definition() -> TaskDefinition {
        TaskDefinition {
            id: Uuid::new_v4(),
            name: "Sample Task".to_string(),
            desc: Some("Sample Description".to_string()),
            start: Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap(),
            recurrence: Some(Recurrence::Recurring {
                last_recurrence: None,
                minutes: 60,
                exceptions: None,
            }),
        }
    }

    #[test]
    fn test_push_and_get_task_definition() {
        let mut reminder = TaskReminder {
            task_definitions: vec![],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        let def = sample_task_definition();
        let id = def.id;
        reminder.push_task_definition(def.clone());

        assert_eq!(
            reminder.get_task_definition(id).unwrap().name,
            "Sample Task"
        );
    }

    #[test]
    fn test_generate_single_instance() {
        let mut reminder = TaskReminder {
            task_definitions: vec![sample_task_definition()],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        reminder.generate_task_instances(0);
        let tasks = reminder.get_tasks(0);

        assert!(!tasks.is_empty());
        assert_eq!(tasks[0].name, "Sample Task");
    }

    #[test]
    fn test_mark_task_completed_updates_last_recurrence() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_data_path: PathBuf = temp_dir.path().join("tasks.json");

        config::init(test_data_path.clone());

        let mut reminder = TaskReminder {
            task_definitions: vec![sample_task_definition()],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        reminder.generate_task_instances(0);
        let task = reminder.get_next_task().unwrap().clone();
        reminder.mark_task_completed(task.clone()).unwrap();

        let updated_def = reminder.get_task_definition(task.definition_id).unwrap();
        if let Some(Recurrence::Recurring {
            last_recurrence, ..
        }) = &updated_def.recurrence
        {
            assert_eq!(*last_recurrence, Some(task.timestamp));
        } else {
            panic!("Expected recurring task");
        }
    }

    #[test]
    fn test_delete_task_definition() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_data_path: PathBuf = temp_dir.path().join("tasks.json");

        config::init(test_data_path.clone());

        let mut reminder = TaskReminder {
            task_definitions: vec![sample_task_definition()],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        let id = reminder.task_definitions[0].id;
        reminder.delete_task_definition(id).unwrap();

        assert!(reminder.get_task_definition(id).is_none());
    }

    #[test]
    fn test_non_recurring_task_generates_single_instance() {
        let mut reminder = TaskReminder {
            task_definitions: vec![TaskDefinition {
                id: Uuid::new_v4(),
                name: "One-time Task".to_string(),
                desc: None,
                start: Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap(),
                recurrence: Some(Recurrence::None),
            }],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        reminder.generate_task_instances(0);
        let tasks = reminder.get_tasks(0);

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "One-time Task");
    }

    #[test]
    fn test_task_with_exception_does_not_generate_that_instance() {
        let start = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let exception_time = start + Duration::minutes(60);

        let mut reminder = TaskReminder {
            task_definitions: vec![TaskDefinition {
                id: Uuid::new_v4(),
                name: "With Exception".to_string(),
                desc: None,
                start,
                recurrence: Some(Recurrence::Recurring {
                    last_recurrence: None,
                    minutes: 60,
                    exceptions: Some(vec![exception_time]),
                }),
            }],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        reminder.generate_task_instances(0);
        let tasks = reminder.get_tasks(0);

        // Ensure that the exception timestamp is not in the generated list
        for task in tasks {
            assert_ne!(task.timestamp, exception_time);
        }
    }

    #[test]
    fn test_pagination_limits_task_output() {
        let def = TaskDefinition {
            id: Uuid::new_v4(),
            name: "Paged Task".to_string(),
            desc: None,
            start: Utc.with_ymd_and_hms(2025, 1, 1, 8, 0, 0).unwrap(),
            recurrence: Some(Recurrence::Recurring {
                last_recurrence: None,
                minutes: 5, // small interval to fill multiple pages quickly
                exceptions: None,
            }),
        };

        let mut reminder = TaskReminder {
            task_definitions: vec![def],
            task_instances: BinaryHeap::new(),
            calculated_instances: 0,
        };

        // Generate page 0
        let page_0 = reminder.get_tasks(0);
        assert_eq!(page_0.len(), PAGE_SIZE as usize);

        // Generate page 1
        let page_1 = reminder.get_tasks(1);
        assert_eq!(page_1.len(), PAGE_SIZE as usize);

        // Ensure no duplicates between pages
        let timestamps_page_0: Vec<_> = page_0.iter().map(|t| t.timestamp).collect();
        let timestamps_page_1: Vec<_> = page_1.iter().map(|t| t.timestamp).collect();
        assert!(timestamps_page_0
            .iter()
            .all(|ts| !timestamps_page_1.contains(ts)));
    }
}
