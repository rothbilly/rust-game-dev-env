use macroquad::prelude::*;
use num_bigint::BigInt;

use crate::core::app::App;


#[derive(Default)]
pub struct Game{
    base_app: App,
    score: u32
}

impl Game {

    /// Create a new game instance
    pub fn new(base_app: App, score: u32) -> Self {
        Self { base_app, score }
    }
    

}


pub trait GameLoop{
    //fn _enter(){}
    fn _ready(&mut self) -> &mut Self;
    fn _process_input(&mut self);
    fn _update(&mut self, _delta: f32);
    fn _render(&mut self);
    //fn _draw(){}
    //fn _stop(){}
    //fn _exit(){}
    //fn _free(&mut self){}
}





