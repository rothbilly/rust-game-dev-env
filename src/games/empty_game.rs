use crate::core::app::AppRegistration; // To register it in the App registry.
use engine_macros::register_app;
use crate::core::game::{GameLoop};
use macroquad::prelude::*;


/// Use this game as a template for new games


#[register_app("EmptyGame")] // Custom Registration macro
#[derive(Default)]

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

