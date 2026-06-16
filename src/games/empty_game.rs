use engine_macros::register_app;
use macroquad::prelude::*;
use crate::core::game::{GameLoop};
use crate::core::app::AppRegistration; // To register it in the App registry.


#[derive(Default)]
#[register_app("EmptyGame")]
pub struct EmptyGame;


impl GameLoop for EmptyGame {
    fn _ready(&mut self) {

    }

    fn _process_input(&mut self) {

    }

    fn _update(&mut self, _delta: f32) {

    }

    fn _render(&self) {

    }
}

