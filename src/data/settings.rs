use std::fs;
use std::path::Path;

use macroquad::prelude::*;
use nanoserde::*;


const PATH_STR: &str = ".app/settings.json";

#[derive(Debug, SerJson, DeJson)]
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


    pub fn load(&mut self) -> Settings {
        if let Ok(contents) = fs::read_to_string(PATH_STR) {

            DeJson::deserialize_json(&contents).unwrap_or_default()
        } else {
            Settings::default()
        }
    }

    pub fn save(&self) {
        if let Some(parent) = Path::new(PATH_STR).parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = self.serialize_json();
        let _ = fs::write(PATH_STR, json);
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