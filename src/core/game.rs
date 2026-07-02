use macroquad::prelude::*;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use crate::components::gameobject::*;

#[derive(Default)]
pub struct Game
{
    pub background_color : Color,
    pub game_objects : HashMap<String, Box<dyn GameObject>>,
    pub game_camera: Option<Box<dyn Camera>>, // Can be a camera 2d pr 3D or None
}

pub trait GameLoop : DerefMut<Target = Game>{

    fn ready(&mut self) {
        self._ready();
    }
    fn process_input(&mut self){
        self._process_input();
    }
    // Engine methods (called automatically)
    fn update(&mut self, delta : f32){
        for (_id, game_object) in &mut self.game_objects {
            game_object._update(delta);
        }
        self._update(delta);
    }

    fn render(&self){
        if let Some(ref camera) = self.game_camera {
            set_camera(camera.as_ref());
        }
        clear_background(self.background_color);

        for (_id, game_object) in &self.game_objects { // Making it an immutable reference prevents destroying!
            if (matches!(game_object.layer(), RenderLayer::World)){
                game_object._draw();
            }
        } 
        set_default_camera();

        for (_id, game_object) in &self.game_objects { // Making it an immutable reference prevents destroying!
            if (matches!(game_object.layer(), RenderLayer::Screen)){
                game_object._draw();
            }
        } 


        self._render();
    }

    // User methods (overrided by user)
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


/// Enforce the struct to have a composition base_app field; 
pub trait IParentable{
    fn get_base_app(&self) -> Option<&Box<dyn IParentable>>;
}

pub trait IPausable {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, _value: bool);
}


/// # The main trait
/// Here every game has to implement this trait.
/// This can be converted in a derive macro as #[derive(BaseGame)] that implement the sub traits

pub trait BaseGame : GameLoop + GameCamera + IParentable + IPausable {}  

pub trait IGame : GameLoop + IParentable + IPausable{}
