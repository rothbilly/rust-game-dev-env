

use macroquack::prelude::*;
use macroquack::core::game::{Game, GameLoop};
use macroquack::core::app::AppRegistration; // To register it in the App registry.

/// Use this game as a template for new games

// Custom Registration macro
#[derive(Default)]
#[register_app] 
#[auto_deref(base_game)]
pub struct EmptyGame{
    base_game: Game
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

