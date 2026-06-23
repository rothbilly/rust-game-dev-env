use engine_macros::register_app;
use macroquad::prelude::*; // To register it in the App registry.
use crate::core::game::GameLoop;
use crate::core::app::AppRegistration;

#[derive(Default)]
#[register_app("Sandbox")] // Custom Registration macro
pub struct Sandbox;

impl GameLoop for Sandbox {
    fn _ready(&mut self) {}

    fn _process_input(&mut self) {}

    fn _update(&mut self, _delta: f32) {}

    fn _render(&self) {}
}
