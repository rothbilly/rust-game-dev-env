use egui_macroquad::egui::Context;


pub mod event_inspector;
pub mod app_loader;


pub trait InspectorWindow: IVisibility + Inspector{}

pub trait IVisibility{
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, _value: bool);
}

// Draw the GUI
pub trait Inspector {

    fn _setup(&mut self){
    }
    fn _draw_gui(&mut self) ->bool{
        false
    }
    fn _draw_egui(&self, _egui_ctx: &Context){
    }
}
