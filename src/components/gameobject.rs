
use macroquad::prelude::*;

// I don't know what's the best practice but I made Draweable and Updateable as independent traits.
// Because even though every gameobject is updateable and draweable, maybe some objects implement different updates?
pub trait Draweable{
    fn _draw(&self);
}

pub trait Updateable{
    fn _update(&mut self, delta : f32);
}

// Multi-type trait aka auto traits. Here should be everything that a GameObject implements.
pub trait GameObject: Draweable + Updateable {
    fn position(&self) -> Vec3;
    fn set_position(&mut self, pos: Vec3);
}

// Base Data.
#[derive(Default)] 
pub struct BaseGameObject {
    pub position : Vec3,
    pub color : Color,
}

// This works like an interface.
pub trait HasBaseGameObject {
    fn base_game_object(&self) -> &BaseGameObject;
    fn base_game_object_mut(&mut self) -> &mut BaseGameObject;
}

// Implement this for every different type of GameObject.
impl<T> GameObject for T
where
    T : HasBaseGameObject + Draweable + Updateable
{
    fn position(&self) -> Vec3{
        self.base_game_object().position
    }

    fn set_position(&mut self, pos : Vec3){
        self.base_game_object_mut().position = pos;
    }
}
// ------------------------------------
// Sphere, a type of GameObject.
#[derive(Default)]
pub struct Sphere{
    pub base_data : BaseGameObject,
    pub radius : f32,
}

impl HasBaseGameObject for Sphere{
    fn base_game_object(&self) -> &BaseGameObject {
        &self.base_data
    }
    fn base_game_object_mut(&mut self) -> &mut BaseGameObject{
        &mut self.base_data
    }
}

impl Draweable for Sphere {
    fn _draw(&self){
        draw_sphere(self.base_data.position, 
                    self.radius, 
                    None, 
                    self.base_data.color);
    }
}

impl Updateable for Sphere {
    fn _update(&mut self, _delta : f32){

    }
}

impl Sphere {
    pub fn new(position : Vec3, color : Color, radius : f32) -> Self {
        Self { base_data : BaseGameObject {position, color}, radius }
    }
}
// -----------------------------------
// WireCube
#[derive(Default)]
pub struct WireCube{
    pub base_data : BaseGameObject,
    pub size : Vec3,
}

impl Draweable for WireCube {
    fn _draw(&self){
        draw_cube_wires(self.base_data.position, 
                        self.size,
                        self.base_data.color);
    }
}

impl Updateable for WireCube {
    fn _update(&mut self, _delta : f32){
        
    }
}

impl HasBaseGameObject for WireCube{
    fn base_game_object(&self) -> &BaseGameObject {
        &self.base_data
    }
    fn base_game_object_mut(&mut self) -> &mut BaseGameObject{
        &mut self.base_data
    }
}
impl WireCube{
    pub fn new(position : Vec3, color : Color, size : Vec3) -> Self {
        Self { base_data : BaseGameObject {position, color}, size }
    }
    fn size(&self) -> Vec3 { self.size }
    fn set_size(&mut self, sz : Vec3) { self.size = sz }
}
