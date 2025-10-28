use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::games::GameState;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SaveData {
    pub scores: Vec<GameState>,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub audio_enabled: bool,
    pub theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            audio_enabled: true,
            theme: Theme::Light,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

pub struct Storage {
    pub path: PathBuf,
}

impl Storage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> SaveData {
        if let Ok(content) = fs::read_to_string(&self.path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            SaveData::default()
        }
    }

    pub fn save(&self, data: &SaveData) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(data) {
            let _ = fs::write(&self.path, json);
        }
    }
}
