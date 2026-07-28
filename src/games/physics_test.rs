use engine_macros::{auto_deref, register_app};
use macroquad::prelude::*;
use crate::core::game::{Game, GameLoop};
use crate::core::app::AppRegistration; // To register it in the App registry.


#[register_app("PhysicsTest")] // Custom Registration macro
#[auto_deref(base)]
#[derive(Default)]

pub struct PhysicsTest{
    base: Game
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