use crate::core::{app::AppRegistration, game::Game}; // To register it in the App registry.
use engine_macros::{auto_deref, register_app};
use crate::core::game::{GameLoop};
use macroquad::prelude::*;


/// Use this game as a template for new games


#[register_app] // Custom Registration macro
#[auto_deref(base)]
#[derive(Default)]
pub struct EmptyGame{
    base: Game
}


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

