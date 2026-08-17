use egui_macroquad::egui::Context;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DrawingBackend {
    #[default]
    Macroquad,
    Egui,
}

pub trait InspectorWindow: IVisibility + IName + Inspector {}

pub trait IName {
    fn get_struct_name(&self) -> String {
        stringify!(type).to_string() //
    }
}

pub trait IVisibility {
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, _value: bool);
}

// Draw the GUI
pub trait Inspector {
    fn _setup(&mut self) {}
    fn _draw_gui(&mut self) -> bool {
        false
    }
    fn _draw_egui(&mut self, _egui_ctx: &Context) {}
}
