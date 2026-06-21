use macroquad::prelude::*;



#[derive(Default)]
pub struct Game
{
    _score: u32
}


/// # The main trait
/// Here every game has to implement this trait.



pub trait  GameLoop{
    fn _ready(&mut self){}
    fn _process_input(&mut self){}
    fn _update(&mut self, _delta: f32){}
    fn _render(&self){}
}





