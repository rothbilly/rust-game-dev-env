use macroquad::prelude::*;
use crate::core::game::{GameLoop};
use crate::core::app::AppRegistration; // To register it in the App registry.
use crate::core::math::wrapf;

const MOVEMENT_SPEED: f32 = 200.0;


#[derive(Default)]

pub struct TutorialGame{
    dir: Vec2,
    pos: Vec2
}


impl GameLoop for TutorialGame {
    fn _ready(&mut self) {
        self.pos.x = screen_width() / 2.0;
        self.pos.y = screen_height() / 2.0;
    }

    fn _process_input(&mut self) {
        self.dir.x = is_key_down(KeyCode::D) as u8 as f32 -  is_key_down(KeyCode::A) as u8 as f32;
        self.dir.y = is_key_down(KeyCode::S) as u8 as f32 -  is_key_down(KeyCode::W) as u8 as f32;
    }

    fn _update(&mut self, _delta: f32) {
        self.pos += self.dir * MOVEMENT_SPEED * _delta;
        self.pos.x = wrapf(self.pos.x, 0.0, screen_width());
        self.pos.y = wrapf(self.pos.y, 0.0, screen_height());
    }

    fn _render(&self) {
        clear_background(DARKPURPLE);
        draw_circle(self.pos.x, self.pos.y, 16.0, YELLOW);
    }
}

// This automatically puts this App, which implements "GameLoop", into "AppRegistry" for posterior use.
inventory::submit! {
    AppRegistration {
        name: "TutorialGame", //set the name here
        constructor: || std::rc::Rc::new(std::cell::RefCell::new(TutorialGame::default())),
    }
}