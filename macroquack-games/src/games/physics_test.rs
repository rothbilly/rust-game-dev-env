use macroquack::engine_macros::register_app;
use macroquad::prelude::*;
use macroquack::core::game::{GameLoop};
use macroquack::core::app::AppRegistration; // To register it in the App registry.


#[register_app("PhysicsTest")] // Custom Registration macro
#[derive(Default)]
pub struct PhysicsTest;

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