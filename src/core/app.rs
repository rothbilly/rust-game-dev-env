
use macroquad::prelude::*;

/// # The App Base Module

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



/// # App Mode 
/// Define if the app is used for 2D or 3D Rendering or as editor UI
#[derive(Default)]
pub enum AppMode {
    #[default]
    Mode2D,
    Mode3D,
    ModeUI
}


