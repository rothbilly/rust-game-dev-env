
use macroquad::color::colors;
use macroquad::prelude::*;

#[derive(Default)]
pub struct GameObject{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub color : Color
}

// I don't know the correct practice, but for now I am splitting the traits.
pub trait Updateable{
    fn _update(&mut self); 
}

pub trait Draweable{
    fn _draw(&mut self);
}

// For testing
impl GameObject{
    pub fn new(x : f32, y : f32, z : f32, color : Color) -> Self {
        Self{ x, y, z, color }
    }

    fn _ready(){
        println!("Hello world!")
    }
}

impl Draweable for GameObject {
    fn _draw(&mut self){
        draw_sphere(vec3(self.x, self.y, self.z), 1., None, self.color);
    }
}
