use macroquad::prelude::*;
use crate::core::game::{Game, GameLoop};




#[derive(Default)]

pub struct EmptyGame;


impl GameLoop for EmptyGame {
    fn _ready(&mut self) {

    }

    fn _process_input(&mut self) {

    }

    fn _update(&mut self, _delta: f32) {

    }

    fn _render(&mut self) {

    }
}