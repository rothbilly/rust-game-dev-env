use macroquack::prelude::*;
use macroquad::{input, prelude::*};
use macroquad::color::colors;

    
use macroquack::core::game::{Game, GameLoop};
use macroquack::core::app::AppRegistration;
use macroquack::components::gameobject::*;

#[register_app] 
#[auto_deref(base_game)]
#[derive(Default)]

pub struct MacrowareExe {
    base_game: Game,
    change_background : bool,
    camera : Camera3D, // Camera could be in its own "substruct"
    camera_dir : Vec3,
    up: bool, // Same as input handling.
    down : bool,
    left: bool,
    right: bool,
}

impl GameLoop for MacrowareExe {
    fn _ready(&mut self){
        self.background_color = LIGHTGRAY;
        self.camera.position = vec3(-20., 15., 0.);
        self.camera.target = vec3(0., 0., 0.);
        self.camera.up = vec3(0., 1., 0.);
        self.camera_dir = Vec3::ZERO;
        self.camera.projection = Projection::Perspective;

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
        let up = self.up;
        let down = self.down;
        let left = self.left;
        let right = self.right;
        
        if self.change_background {
            self.background_color = match self.background_color {
                LIGHTGRAY => PURPLE,
                PURPLE => LIGHTGRAY,
                _ => self.background_color
            };
        }
        

        let sphere = self.game_objects.get_mut("sphere").unwrap();
        let mut position = sphere.position();
        if up {
            position.x += 10. * delta;
        }
        if down {
            position.x -= 10. * delta;
        }
        if left{
            position.z -= 10. * delta;
        }
        if right{
            position.z += 10. * delta;
        }
        sphere.set_position(position);
    }

    
    fn _render(&self) {
        set_camera(&self.camera);
        draw_grid(20, 1., BLACK, GRAY);
        // set_default_camera(); // for 2d
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


