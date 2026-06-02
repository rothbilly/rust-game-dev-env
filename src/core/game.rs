use macroquad::prelude::*;



#[derive(Default)]
pub struct Game

{
    score: u32
}


pub trait  GameLoop{
    fn _ready(&mut self);
    fn _process_input(&mut self);
    fn _update(&mut self, _delta: f32);
    fn _render(&mut self);
}





