use once_cell::sync::OnceCell;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub data_path: PathBuf,
}

static CONFIG: OnceCell<Config> = OnceCell::new();
static TASK_FILE_NAME: &str = "tasks.json";

fn setup_data(dir_path: &PathBuf) -> PathBuf {
    if !dir_path.exists() {
        println!("creating dir");
        fs::create_dir(dir_path).expect("Failed to create task file directory");
    }

    let task_path = dir_path.join(TASK_FILE_NAME);
    if !task_path.exists() {
        println!("creating file at {:?}", task_path);
        println!("writing to file");
        fs::write(&task_path, "[]").expect("Failed to initialize task file to valid json");
    }

    task_path
}

pub fn get() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}

pub fn init(data_dir_path: PathBuf) {
    println!("DATA PATH: {:?}", data_dir_path);

    let full_path = setup_data(&data_dir_path);
    // Needs to be called before the path is set on the config

    CONFIG
        .set(Config {
            data_path: full_path,
        })
        .expect("Config already initialized");
}
