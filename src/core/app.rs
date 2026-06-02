
use macroquad::prelude::*;

use crate::core::game::GameLoop;

/// # The App Base Module

#[derive(Default)]
pub struct App
{
    title: String,
    mode: AppMode,
    is_active: bool,
    rect: Rect,
    sub_apps: Vec<Box<dyn GameLoop>>

}

impl App
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn _initialize(&mut self){

    }

    pub fn initialize_and_ready<T>() -> App
    where T:GameLoop + Default + 'static
    {
        let mut app = App::default();
        app.sub_apps.push(Box::new(T::default()));
        app._ready();
        app
    }

    pub fn append_new_sub_app<T>(&mut self) -> &mut Self
    where T:GameLoop + Default + 'static{
        self.sub_apps.push(Box::new(T::default()));
        if let Some(new_app) = self.sub_apps.last_mut(){
            new_app._ready();
        }
        self
    }

    pub fn _ready(&mut self) -> &mut Self{
        for app in self.sub_apps.iter_mut(){
            app._ready();
        }
        self
    }

    /// the application proccess ordering
    pub fn _runtime(&mut self , delta_time: f32){
        if self.sub_apps.len() == 0 {return}

        for app in self.sub_apps.iter_mut(){
            app._process_input();
            app._update(delta_time);
            app._render();
        }

    }

    pub fn is_main_app(&self) -> bool{
        self.sub_apps.len() != 0 
    }
}




/// # App Mode 
/// Define if the app is used for 2D or 3D Rendering or as editor UI
#[derive(Default)]
pub enum AppMode {
    #[default]
    Mode2D,
    Mode3D,
    ModeUI,
    Headless
}


