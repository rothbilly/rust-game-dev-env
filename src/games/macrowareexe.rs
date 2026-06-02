use macroquad::{input, prelude::*};
use macroquad::color::colors;
use macroquad::ui::{hash, root_ui, widgets::Window};

use egui_macroquad::*;

use crate::core::game::{GameLoop};
use crate::{editor::constants::*};
use crate::editor::inspector::EventInspector;

use crate::components::gameobject::*;

use std::collections::HashMap;

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
    game_objects : HashMap<String, Box<dyn GameObject>>,
}

impl GameLoop for MacrowareExe {
    fn _ready(&mut self){
        self.background_color = LIGHTGRAY;
        self.camera.position = vec3(-20., 15., 0.);
        self.camera.target = vec3(0., 0., 0.);
        self.camera.up = vec3(0., 1., 0.);
        self.camera_dir = Vec3::ZERO;
        self.camera.projection = Projection::Perspective;
        self.game_objects = HashMap::new();

        let sphere = Box::new(Sphere::new(vec3(-8., 0., 0.), BLUE, 1.0));
        let sphere2 = Box::new(Sphere::new(vec3(-4., 0., 0.), RED, 2.0));
        let cube = Box::new(WireCube::new(vec3(0., 1., -6.), DARKGREEN, vec3(2., 2., 2.)));
        let cube2 = Box::new(WireCube::new(vec3(0., 1., 6.), DARKBLUE, vec3(2., 2., 2.)));
        let cube3 = Box::new(WireCube::new(vec3(2., 1., 2.), YELLOW, vec3(2., 2., 2.)));


        self.game_objects.insert("sphere".to_string(), sphere);
        self.game_objects.insert("sphere2".to_string(), sphere2);
        self.game_objects.insert("cube1".to_string(), cube);
        self.game_objects.insert("cube2".to_string(), cube2);
        self.game_objects.insert("cube3".to_string(), cube3);
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

        for (id, game_object) in &mut self.game_objects {
            game_object._update(delta);
        }


        let sphere = self.game_objects.get_mut("sphere").unwrap();
        let mut position = sphere.position();
        if self.up {
            position.x += 10. * delta;
        }
        if self.down {
            position.x -= 10. * delta;
        }
        if self.left{
            position.z -= 10. * delta;
        }
        if self.right{
            position.z += 10. * delta;
        }
        sphere.set_position(position);
    }

    fn _render(& mut self) {
        clear_background(self.background_color);
        set_camera(&self.camera);
        draw_grid(20, 1., BLACK, GRAY);


        for (id, game_object) in &self.game_objects { // Making it an immutable reference prevents destroying!
            game_object._draw();
        } 

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
    }
}
