use macroquad::{math::vec2, ui::{hash, root_ui, widgets::{self, Window},}};

use macroquad::prelude::*;


pub struct EventInspector;

impl EventInspector {
    pub fn new() -> Self {

        return EventInspector
    }


    pub fn draw() -> bool {
        let window_status = Window::new(hash!(), vec2(20.,20.), vec2(360., 640.))
        .label("Event Menu")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut *root_ui(), |ui|{
            let (mouse_x, mouse_y) = mouse_position();
            ui.label(None, &format!("Mouse Position: {mouse_x} {mouse_y}"));
            
            let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
            ui.label(None, &format!("Mouse wheel x: {mouse_wheel_x}"));
            ui.label(None, &format!("Mouse wheel y: {mouse_wheel_y}"));

            widgets::Group::new(hash!(), Vec2::new(200.,0.))
                .position(Vec2::new(240.,0.))
                .ui(ui,|ui|{
                    ui.label(None, "Pressed kbd keys");

                    if let Some(key) = get_last_key_pressed() {
                        ui.label(None, &format!("{key:?}"))
                    }
                });

    });
    return window_status;
    }
}
