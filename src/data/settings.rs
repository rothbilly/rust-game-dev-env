use std::fs;
use std::path::Path;

use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

const PATH_STR: &str = ".app/settings.json";
const TOML_PATH_STR: &str = ".app/settings.toml";
const BIN_PATH_STR: &str = ".app/settings.bin";
const RON_PATH_STR: &str = ".app/settings.ron";

pub enum SerializeMode {
    JSON,
    TOML,
    BIN,
    RON,
}

// Derive is "automatic" code. Using default implementations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    volume: f32,
    pub window_h: i32,
    pub window_w: i32,
}

// This is akin to an empty constructor.
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

    /// Load settings from the binary file (".app/settings.bin").
    /// Falls back to `Settings::default()` on read/deserialize errors.
    pub fn load_bin(&mut self) -> Settings {
        Self::load_from_bin_path(BIN_PATH_STR)
    }

    /// Load settings from a custom binary file path (accepts `String`, `&str`, or `&Path`).
    pub fn load_from_bin_path<P: AsRef<Path>>(path: P) -> Settings {
        if let Ok(bytes) = fs::read(path.as_ref()) {
            match bincode::deserialize::<Settings>(&bytes) {
                Ok(s) => s,
                Err(_) => Settings::default(),
            }
        } else {
            Settings::default()
        }
    }

    /// Save settings to the binary file (".app/settings.bin").
    pub fn save_bin(&self) {
        self.save_bin_to_path(BIN_PATH_STR)
    }

    /// Save settings to a custom binary file path (accepts `String`, `&str`, or `&Path`).
    pub fn save_bin_to_path<P: AsRef<Path>>(&self, path: P) {
        if let Some(parent) = path.as_ref().parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(bytes) = bincode::serialize(self) {
            let _ = fs::write(path.as_ref(), bytes);
        }
    }

    /// Load settings from the RON file (".app/settings.ron").
    /// Falls back to `Settings::default()` on read/parse errors.
    pub fn load_ron(&mut self) -> Settings {
        Self::load_from_ron_path(RON_PATH_STR)
    }

    /// Load settings from a custom RON file path (accepts `String`, `&str`, or `&Path`).
    pub fn load_from_ron_path<P: AsRef<Path>>(path: P) -> Settings {
        if let Ok(contents) = fs::read_to_string(path.as_ref()) {
            ron::from_str::<Settings>(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
        }
    }

    /// Save settings to the RON file (".app/settings.ron").
    pub fn save_ron(&self) {
        self.save_ron_to_path(RON_PATH_STR)
    }

    /// Save settings to a custom RON file path (accepts `String`, `&str`, or `&Path`).
    pub fn save_ron_to_path<P: AsRef<Path>>(&self, path: P) {
        if let Some(parent) = path.as_ref().parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(ron_str) = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()) {
            let _ = fs::write(path.as_ref(), ron_str);
        }
    }

    /// Convenience: save using the given mode to default locations.
    pub fn save_with_mode(&self, mode: SerializeMode) {
        match mode {
            SerializeMode::JSON => self.save(),
            SerializeMode::TOML => self.save_toml(),
            SerializeMode::BIN => self.save_bin(),
            SerializeMode::RON => self.save_ron(),
        }
    }

    /// Convenience: save using the given mode to a custom path.
    pub fn save_with_mode_to<P: AsRef<Path>>(&self, mode: SerializeMode, path: P) {
        match mode {
            SerializeMode::JSON => self.save_to_json_path(path),
            SerializeMode::TOML => self.save_toml_to_path(path),
            SerializeMode::BIN => self.save_bin_to_path(path),
            SerializeMode::RON => self.save_ron_to_path(path),
        }
    }

    // Getters. Notice that it does not return a reference.
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
