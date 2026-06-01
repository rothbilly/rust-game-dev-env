use macroquad::prelude::*;
use num_bigint::BigInt;

use crate::core::app::{App, VirtualApp};


#[derive(Default)]
pub struct Game{
    base_app: VirtualApp,
    score: u32
}

impl Game {
}


pub trait GameLoop{
    //fn _enter(){}
    fn _ready(&mut self) -> &mut Self;
    fn _process_input(&mut self);
    fn _update(&mut self, _delta: f32);
    fn _render(&mut self);
}





