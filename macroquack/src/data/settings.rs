use std::path::Path;
use std::sync::RwLock;
use std::{fs, sync::LazyLock};

use macroquad::prelude::*;

use crate::data::serialization::*;
use crate::editor::inspector::DrawingBackend;
use serde::{Deserialize, Serialize};

const TOML_PATH_STR: &str = ".app/settings.toml";

/// # Engine global settings
/// static = eternal lifetime
/// LazyLock = delays execution until it is called on code. Avoids having to define static at compile time.
/// Mutex = Can only be edited under a lock. Avoids race conditions.
pub static GLOB_SETTINGS: LazyLock<RwLock<Settings>> =
    LazyLock::new(|| RwLock::new(Settings::new()));

// Derive is "automatic" code. Using default implementations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    volume: f32,
    pub window_h: i32,
    pub window_w: i32,
    pub egui_render_scale: f32,
    editor_renderer: DrawingBackend,
    pub fullscreen: bool,
    pub swap_interval: i32, //GPU hint, not reliable, debug only
    pub starting_app: String,
}

// This is akin to an empty constructor.
impl Default for Settings {
    fn default() -> Self {
        Settings {
            volume: 0.5,
            window_h: 720,
            window_w: 1280,
            egui_render_scale: 0.8,
            editor_renderer: DrawingBackend::Macroquad,
            fullscreen: false,
            swap_interval: 1,
            starting_app: "Tutorial Game".to_string(),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        info!("[Settings] - lazy initialization");
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn load_toml(&mut self) -> &mut Self {
        info!("[Settings] - loading settings");
        //a deref here, consumes the old value and set self to the new value, not the pointer;
        *self = Settings::load_from_toml_path(TOML_PATH_STR);
        self
    }

    /// Load settings from a custom TOML file path (accepts `String`, `&str`, or `&Path`).
    pub fn load_from_toml_path<P: AsRef<Path>>(path: P) -> Settings {
        if let Ok(contents) = fs::read_to_string(path.as_ref())
            && let Ok(setting) = toml::from_str::<Settings>(&contents)
        {
            return setting;
        }
        Settings::default()
    }

    pub fn window_size(&self) -> (i32, i32) {
        (self.window_w, self.window_h)
    }
}

impl ISerializableData for Settings {}
