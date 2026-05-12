
use macroquad::prelude::*;



#[derive(Default)]
pub enum AppMode {
    #[default]
    Mode2D,
    Mode3D,
    ModeUI
}


#[derive(Default)]
pub struct App{
    title: String,
    mode: AppMode,
    is_active: bool,
    is_focused: bool,
    rect: Rect
}

impl App {
    pub fn new(title: String, mode: AppMode,  is_active: bool, is_focused: bool, rect: Rect) -> Self {
        Self { title, mode, is_active, is_focused, rect }
    }
}


trait Child{
    fn get_parent_struct<T>(&mut self) -> Option<T>;
} 