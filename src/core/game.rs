use macroquad::prelude::*;



#[derive(Default)]
pub struct Game
{
    _score: u32
}


/// # The main trait
/// Here every game has to implement this trait.

trait BaseGame : GameLoop + GameCamera{}  

pub trait  GameLoop{
    fn _ready(&mut self){}
    fn _process_input(&mut self){}
    fn _update(&mut self, _delta: f32){}
    fn _render(&self){}
}


type SomeCamera = Option<Box<dyn Camera>>;

pub trait GameCamera{
    fn try_get_camera(&self) -> SomeCamera{
        None
    }

    fn has_camera(&self) -> bool;

    fn set_render_target(&mut self);
}




