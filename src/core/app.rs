
use egui_macroquad::egui::TextStyle::Body;
use macroquad::prelude::*;

use crate::{core::game::{Game, GameLoop}, games::MacrowareExe};

/// # The App Base Module

#[derive(Default)]
pub struct App<T>
where T :  GameLoop + Default + 'static
{
    title: String,
    mode: AppMode,
    is_active: bool,
    rect: Rect,
    sub_apps: Vec<Box<T>>
}

impl<T> App<T>
where T: GameLoop + Default + 'static
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn _initialize(&mut self){

    }

    pub fn initialize_and_ready() -> App<T>
    {
        let mut app = App::default();
        app.sub_apps.push(Box::new(T::default()));
        app._ready();
        app
    }

    pub fn _ready(&mut self) -> &mut Self{
        for app in self.sub_apps.iter_mut(){
            app._ready();
        }
        self
    }

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



#[derive(Default)]
pub struct VirtualApp;

impl GameLoop for VirtualApp {
    fn _ready(&mut self) -> &mut Self {
        self
    }

    fn _process_input(&mut self) {
        todo!()
    }

    fn _update(&mut self, _delta: f32) {
        todo!()
    }

    fn _render(&mut self) {
        todo!()
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


