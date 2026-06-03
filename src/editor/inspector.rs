
use macroquad::{math::vec2, ui::{Layout, hash, root_ui, widgets::{Group, Window}}};

use macroquad::prelude::*;

use crate::{editor::constants::*};



#[derive(Default, Hash)]
pub struct EventInspector{

    is_visible: bool
}

impl Inspector for EventInspector {
    fn _draw_gui(&self) -> bool{
        let window_status = Window::new(hash!(), vec2(GLOB_MARGIN, GLOB_MARGIN), vec2(PANEL_WIDTH, PANEL_HEIGHT))
        .label("Event Menu")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut root_ui(), |ui|{
            let (mouse_x, mouse_y) = mouse_position();
            ui.label(None, &format!("Mouse Position: {mouse_x} {mouse_y}"));
            
            let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
            ui.label(None, &format!("Mouse wheel x: {mouse_wheel_x}"));
            ui.label(None, &format!("Mouse wheel y: {mouse_wheel_y}"));

            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
                .layout(Layout::Vertical)
                .ui(ui,|ui|{
                    ui.label(None, "Pressed keyboard keys");

                    if let Some(key) = get_last_key_pressed() {
                        ui.label(None, &format!("{key:?}"))
                    }
                    if ui.button( None,"test button"){
                        println!("a button well made");
                    }
                });

            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
            .layout(Layout::Vertical)
            .ui(ui, |ui|{

                //let mut skin = root_ui().default_skin().clone();

                ui.label(None, "Pressed mouse keys");
                
            
                if is_mouse_button_down(MouseButton::Left) {
                        ui.label(None, "Left");
                    }
                if is_mouse_button_down(MouseButton::Right) {
                        ui.label(None, "Right");
                    }
                if is_mouse_button_down(MouseButton::Middle) {
                        ui.label(None, "Middle");
                    }
            });

    });
    window_status
    }
}


impl IVisibility for EventInspector{
    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visible(&mut self, _value: bool) {
        self.is_visible = _value
    }
}

impl InspectorWindow for EventInspector{}



pub trait InspectorWindow: IVisibility + Inspector{}

pub trait IVisibility{
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, _value: bool);
}


pub trait Inspector {
    fn _draw_gui(&self) ->bool{
        false
    }

    fn _draw_egui(&self){

    }
}
