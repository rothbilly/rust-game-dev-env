use macroquad::prelude::*;
use macroquack::prelude::*;
use macroquack::core::game::{Game, GameLoop};
use macroquack::core::app::AppRegistration; // To register it in the App registry.


#[register_app] 
#[auto_deref(base_game)]
#[derive(Default)]
pub struct PhysicsTest{
    base_game: Game
}

impl GameLoop for PhysicsTest {
    fn _ready(&mut self) {
        self.background_color = LIGHTGRAY;
    }

    fn _process_input(&mut self) {
    }

    fn _update(&mut self, _delta: f32) {
    }

    fn _render(&self) {
    }
}