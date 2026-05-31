use macroquad::{input, prelude::*};
use macroquad::color::colors;
use macroquad::ui::{hash, root_ui, widgets::Window};

use egui_macroquad::*;

use crate::core::game::{GameLoop};
use crate::{editor::constants::*};
use crate::editor::inspector::EventInspector;

use crate::gameobjects::gameobject::*;

#[derive(Default)]
pub struct MacrowareExe {
    background_color : Color,
    change_background : bool,
    camera : Camera3D, // Camera could be in its own "substruct"
    camera_dir : Vec3,
    up: bool, // Same as input handling.
    down : bool,
    left: bool,
    right: bool,
    test : GameObject
}

impl GameLoop for MacrowareExe {
    fn _ready(&mut self) -> &mut Self {
        self.background_color = LIGHTGRAY;
        self.camera.position = vec3(-20., 15., 0.);
        self.camera.target = vec3(0., 0., 0.);
        self.camera.up = vec3(0., 1., 0.);
        self.camera_dir = Vec3::ZERO;
        self.camera.projection = Projection::Perspective;
        
        self.test = GameObject::new(-8., 0., 0., BLUE);

        self
    }

    fn _process_input(&mut self) { 
        self.change_background = is_key_pressed(KeyCode::Space);
        self.up = is_key_down(KeyCode::W);
        self.left = is_key_down(KeyCode::A);
        self.down = is_key_down(KeyCode::S);
        self.right = is_key_down(KeyCode::D);
    }

    fn _update(&mut self, delta : f32) {
        if self.change_background {
            self.background_color = match self.background_color {
                LIGHTGRAY => PURPLE,
                PURPLE => LIGHTGRAY,
                _ => self.background_color
            };
        }

        if self.up {
            self.test.x += 10. * delta;
        }
        if self.down {
            self.test.x -= 10. * delta;
        }
        if self.left{
            self.test.z -= 10. * delta;
        }
        if self.right{
            self.test.z += 10. * delta;
        }
        // self.update_camera(delta);
    }

    fn _render(& mut self) {
        clear_background(self.background_color);
        set_camera(&self.camera);
        draw_grid(20, 1., BLACK, GRAY);
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);
        self.test._draw();
        // draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        set_default_camera(); // for 2d
        draw_hexagon(
            mouse_position().0,
            mouse_position().1,
            if input::is_mouse_button_down(MouseButton::Left) {
                8.
            } else {
                6.
            },
            2.,
            input::is_mouse_button_down(MouseButton::Left),
            if input::is_mouse_button_down(MouseButton::Left) {
                colors::DARKBLUE
            } else {
                DARKBLUE
            },
            if input::is_mouse_button_down(MouseButton::Left) {
                BLUE
            } else {
                DARKBLUE
            },
        );
        let mut input_01 = String::from("sample");
        self.draw_editor_space(&mut input_01);
        self.draw_egui();
    }
}

impl MacrowareExe
{
    fn update_camera(&mut self, delta : f32) {
        self.camera_dir = Vec3::ZERO;

        if self.up {
            self.camera_dir.x += 100.0 * delta
        }

        if self.down {
            self.camera_dir.x -= 100.0 * delta
        }

        if self.left {
            self.camera_dir.z += 100.0 * delta
        }

        if self.right {
            self.camera_dir.z -= 100.0 * delta
        }

        self.camera.position += self.camera_dir * CAM_VELOCITY;
    }

    fn draw_editor_space(&mut self, _input: &mut String) {
        EventInspector::draw();

        let _window_status = Window::new(
            hash!(),
            vec2(1280. - PANEL_WIDTH - GLOB_MARGIN, 20.),
            vec2(PANEL_WIDTH, 100.),
        )
        .label("Inspector")
        .movable(true)
        .close_button(true)
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            ui.editbox(hash!(), vec2(PANEL_WIDTH - PANEL_MARGIN, 80.), _input);

            ui.get_bool(hash!("Inspector"));
        });
    }

    fn draw_egui(&mut self) {
        //egui sample
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui  macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
        });
        });
        egui_macroquad::draw();
    }

}