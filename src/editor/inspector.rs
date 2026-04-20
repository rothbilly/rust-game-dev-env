use macroquad::{math::vec2, ui::{Layout, hash, root_ui, widgets::{Group, Window}, Skin}};

use macroquad::prelude::*;

const PANEL_WIDTH : f32 = 240.;
const PANEL_HEIGHT : f32 = 480.;
const PANEL_MARGIN : f32 = 3.;

pub struct EventInspector;

impl EventInspector {
    pub fn new() -> Self {

        return EventInspector
    }


    pub fn draw() -> bool {
        let window_status = Window::new(hash!(), vec2(20.,20.), vec2(PANEL_WIDTH, PANEL_HEIGHT))
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

            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
                .layout(Layout::Vertical)
                .ui(ui,|ui|{
                    ui.label(None, "Pressed keyboard keys");

                    if let Some(key) = get_last_key_pressed() {
                        ui.label(None, &format!("{key:?}"))
                    }
                });

            Group::new(hash!(), Vec2::new(PANEL_WIDTH - 2.*PANEL_MARGIN,90.))
            .layout(Layout::Vertical)
            .ui(ui, |ui|{

                let mut skin = root_ui().default_skin().clone();
                skin.label_style
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
    return window_status;
    }
}
