

use std::{cell::RefCell, sync::RwLock};

pub use engine_macros::IName;
use engine_macros::IVisibility;

use crate::{data::settings::Settings, editor::inspector::{IVisibility, Inspector, InspectorWindow}};

#[derive(IName)]
#[derive(IVisibility)]
pub struct SettingsInspector{
    window_name: String,
    is_visible: bool,
    panel_height: f32,
    settings: RwLock<Settings>
}

impl InspectorWindow for SettingsInspector {}


impl Inspector for SettingsInspector {
    fn _setup(&mut self){
    }

    fn _draw_gui(&mut self) ->bool{
        false
    }

    fn _draw_egui(&mut self, _egui_ctx: &egui_macroquad::egui::Context){

    }
}

