use std::fs;
use std::path::Path;

use macroquad::prelude::*;

use serde::{Deserialize, Serialize};
use crate::data::serialization::*;

const TOML_PATH_STR: &str = ".app/settings.toml";


// Derive is "automatic" code. Using default implementations.
#[derive(Serialize, Deserialize)]
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

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        self.save_with_mode(SerializeMode::TOML, TOML_PATH_STR)?;
        Ok(())
    }

    /// Load settings from a custom JSON file path (accepts `String`, `&str`, or `&Path`).
    pub fn load_from_json_path<P: AsRef<Path>>(path: P) -> Settings {
        if let Ok(contents) = fs::read_to_string(path.as_ref()) {
            serde_json::from_str(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
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


impl ISerializableData for Settings{

}