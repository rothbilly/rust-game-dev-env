

use std::{fs::TryLockError, ops::Deref, sync::RwLock};

use egui_macroquad::egui::{self, Align2, mutex::RwLockWriteGuard};
pub use engine_macros::IName;
use engine_macros::IVisibility;


use macroquad::prelude::error;

use crate::{data::settings::{self, GLOB_SETTINGS, Settings}, editor::inspector::{IVisibility, Inspector, InspectorWindow}};


#[derive(Default, IName, IVisibility)]
pub struct SettingsInspector{
    window_name: String,
    is_visible: bool,
    panel_height: f32,
}

impl InspectorWindow for SettingsInspector {}


impl Inspector for SettingsInspector {
    fn _setup(&mut self){
        self.window_name = "settings".to_string();
        self.is_visible = true;
    }

    fn _draw_gui(&mut self) ->bool{
        false
    }

    fn _draw_egui(&mut self, _egui_ctx: &egui_macroquad::egui::Context){
        let settings_lock = GLOB_SETTINGS.try_write();
        match settings_lock {
            Ok(guard) => {
                let mut settings = &*guard;

                egui::Window::new(&self.window_name)
                .anchor(Align2::RIGHT_TOP, [-30.,30.])
                .title_bar(true)
                .open( &mut self.is_visible)
                .show(_egui_ctx, |ui|{

                });

            }
            Err(TryLockError) => {
                error!("Settings are locked from another reference")
            }
        }
        
    }
}

