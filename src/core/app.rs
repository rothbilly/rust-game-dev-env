
use macroquad::prelude::*;

use crate::core::game::{Game, GameLoop};

/// # The App Base Module

#[derive(Default)]
pub struct App{
    title: String,
    mode: AppMode,
    is_active: bool,
    rect: Rect,
    sub_apps: Vec<Box<GameLoop + Default + 'static>>
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn _initialize(&mut self){

    }
    pub fn _runtime(&mut self , delta_time: f32){

    }

    pub fn is_main_app(&self) -> bool{
        self.sub_apps.len() == 0
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


