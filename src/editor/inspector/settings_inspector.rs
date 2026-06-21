



use egui_macroquad::egui::{self, Align2, Modal};
pub use engine_macros::IName;
use engine_macros::IVisibility;


use macroquad::{math::clamp, prelude::error};

use crate::{data::settings::{self, GLOB_SETTINGS, Settings}, editor::inspector::{IVisibility, Inspector, InspectorWindow}};


#[derive(Default, IName, IVisibility)]
pub struct SettingsInspector{
    window_name: String,
    is_visible: bool,
    is_value_changed_flag: bool
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
        let mut settings_lock = GLOB_SETTINGS.write().unwrap();
  


        egui::Window::new(&self.window_name)
            .anchor(Align2::RIGHT_TOP, [-30.,30.])
            .title_bar(true)
            .open( &mut self.is_visible)
            .show(_egui_ctx, |ui|{
                ui.horizontal(|ui|{

                    ui.label("Egui Render Scale");
                    if ui.button("<").clicked(){
                        settings_lock.egui_render_scale  = clamp(settings_lock.egui_render_scale - 0.1, 0.1, 2.0);
                        self.is_value_changed_flag = true
                    }
                    ui.add_enabled( false, egui::DragValue::new(&mut settings_lock.egui_render_scale));
                    if ui.button(">").clicked(){
                        settings_lock.egui_render_scale  = clamp(settings_lock.egui_render_scale + 0.1, 0.1, 2.0);
                        self.is_value_changed_flag = true
                    }
                    
                });

                if self.is_value_changed_flag {
                    if ui.button("Save Settings").clicked() {
                        if let Ok(_) =  settings_lock.save() {
                            self.is_value_changed_flag = false
                        }
                    }
                }
            });

        
        
    }

}

