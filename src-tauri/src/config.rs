use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub autostart: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { autostart: true }
    }
}

#[derive(Debug)]
pub struct Config {
    pub data_path: PathBuf,
    pub settings: Settings,
}

static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();
static TASK_FILE_NAME: &str = "tasks.json";
static SETTINGS_FILE_NAME: &str = "settings.json";

fn setup_config(dir_path: &PathBuf) -> (PathBuf, Settings) {
    if !dir_path.exists() {
        fs::create_dir(dir_path).expect("Failed to data directory");
    }

    let task_path = dir_path.join(TASK_FILE_NAME);
    let settings_path = dir_path.join(SETTINGS_FILE_NAME);

    if !task_path.exists() {
        fs::write(&task_path, "[]").expect("Failed to initialize task file to valid json");
    }

    let settings = if settings_path.exists() {
        fs::read_to_string(&settings_path)
            .ok()
            .and_then(|s| serde_json::from_str::<Settings>(&s).ok())
            .unwrap_or_else(Settings::default)
    } else {
        let default_settings = Settings::default();
        fs::write(
            settings_path,
            serde_json::to_string(&default_settings).unwrap(),
        )
        .unwrap();

        default_settings
    };

    println!("Loaded settings: {:?}", settings);

    (task_path, settings)
}

pub fn get() -> MutexGuard<'static, Config> {
    CONFIG
        .get()
        .expect("Config not initialized")
        .lock()
        .unwrap()
}

pub fn update_settings(settings: Settings) -> Result<(), Box<dyn Error>> {
    let mut config = get();
    config.settings = settings;

    let settings_path = config.data_path.parent().unwrap().join("settings.json");

    fs::write(
        settings_path,
        serde_json::to_string(&config.settings).unwrap(),
    )?;

    Ok(())
}

pub fn init(data_dir_path: PathBuf) {
    let (data_path, settings) = setup_config(&data_dir_path);

    CONFIG
        .set(Mutex::new(Config {
            data_path,
            settings,
        }))
        .expect("Config already initialized");
}
