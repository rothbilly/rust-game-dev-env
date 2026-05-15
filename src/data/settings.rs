use std::fs;
use std::path::Path;

use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

const PATH_STR: &str = ".app/settings.json";
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

    /// Load settings from the JSON file (".app/settings.json").
    /// Falls back to `Settings::default()` on read/parse errors.
    pub fn load(&mut self) -> Settings {
        Self::load_from_json_path(PATH_STR)
    }

    /// Load settings from a custom JSON file path (accepts `String`, `&str`, or `&Path`).
    pub fn load_from_json_path<P: AsRef<Path>>(path: P) -> Settings {
        if let Ok(contents) = fs::read_to_string(path.as_ref()) {
            serde_json::from_str(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
        }
    }

    /// Save settings to the JSON file (".app/settings.json").
    pub fn save(&self) {
        self.save_to_json_path(PATH_STR)
    }

    /// Save settings to a custom JSON file path (accepts `String`, `&str`, or `&Path`).
    pub fn save_to_json_path<P: AsRef<Path>>(&self, path: P) {
        if let Some(parent) = path.as_ref().parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path.as_ref(), json);
        }
    }

    /// Load settings from the TOML file (".app/settings.toml").
    /// Falls back to `Settings::default()` on read/parse errors.
    pub fn load_toml(&mut self) -> Settings {
        Self::load_from_toml_path(TOML_PATH_STR)
    }

    /// Load settings from a custom TOML file path (accepts `String`, `&str`, or `&Path`).
    pub fn load_from_toml_path<P: AsRef<Path>>(path: P) -> Settings {
        if let Ok(contents) = fs::read_to_string(path.as_ref()) {
            toml::from_str(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
        }
    }

    /// Save settings to the TOML file (".app/settings.toml").
    pub fn save_toml(&self) {
        self.save_toml_to_path(TOML_PATH_STR)
    }

    /// Save settings to a custom TOML file path (accepts `String`, `&str`, or `&Path`).
    pub fn save_toml_to_path<P: AsRef<Path>>(&self, path: P) {
        if let Some(parent) = path.as_ref().parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(toml_str) = toml::to_string_pretty(self) {
            let _ = fs::write(path.as_ref(), toml_str);
        }
    }

    /// Convenience: save using the given mode to default locations.
    pub fn save_with_mode(&self, mode: SerializeMode) {
        match mode {
            SerializeMode::JSON => self.save(),
            SerializeMode::TOML => self.save_toml(),
            _ => self.save(),
        }
    }

    /// Convenience: save using the given mode to a custom path.
    pub fn save_with_mode_to<P: AsRef<Path>>(&self, mode: SerializeMode, path: P) {
        match mode {
            SerializeMode::JSON => self.save_to_json_path(path),
            SerializeMode::TOML => self.save_toml_to_path(path),
            _ => self.save(),
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
