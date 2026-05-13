use std::fs;
use std::path::Path;

use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

const JSON_PATH_STR: &str = ".app/settings.json";
const TOML_PATH_STR: &str = ".app/settings.toml";

pub enum SerializeMode {
    JSON,
    TOML,
    BIN,
    RON,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    volume: f32,
    pub window_h: i32,
    pub window_w: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new()
    }
}

impl Settings {
    pub const fn new() -> Self {
        Settings {
            volume: 0.5,
            window_h: 720,
            window_w: 1280,
        }
    }

    /// Save settings to the JSON file (pretty-printed). Any IO/serialization errors are ignored.
    pub fn save_as_json(&self) {
        if let Some(parent) = Path::new(JSON_PATH_STR).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(JSON_PATH_STR, json);
        }
    }

    /// Save settings to the TOML file (pretty-printed). Any IO/serialization errors are ignored.
    pub fn save_as_toml(&self) {
        if let Some(parent) = Path::new(TOML_PATH_STR).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(toml_str) = toml::to_string_pretty(self) {
            let _ = fs::write(TOML_PATH_STR, toml_str);
        }
    }


        /// Load settings from the JSON file (".app/settings.json").
    /// Falls back to `Settings::default()` on read/parse errors.
    pub fn load_from_json(&self) -> Settings {
        if let Ok(contents) = fs::read_to_string(JSON_PATH_STR) {
            serde_json::from_str(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
        }
    }

    /// Static helper: read and deserialize the TOML file.
    pub fn load_from_toml(&self) -> Settings {
        if let Ok(contents) = fs::read_to_string(TOML_PATH_STR) {
            toml::from_str(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
        }
    }

    /// Convenience: save using the given mode
    pub fn save_with_mode(&self, mode: SerializeMode) {
        match mode {
            SerializeMode::JSON => self.save_as_json(),
            SerializeMode::TOML => self.save_as_toml(),
            _ => self.save_as_json(),
        }
    }

    pub fn load_with_mode(&self, mode: SerializeMode) -> Settings{
        match mode {
            SerializeMode::JSON => self.load_from_json(),
            SerializeMode::TOML => self.load_from_toml(),
            _ => self.load_from_json(),
        }
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn window_size(&self) -> (i32, i32) {
        (self.window_w, self.window_h)
    }
}
