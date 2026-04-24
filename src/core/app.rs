use macroquad::prelude::*;



#[derive(Default)]
pub struct App{
    title: String,
    is_active: bool,
    is_focused: bool,
    rect: Rect
}

impl App {
    pub fn new(title: String, is_active: bool, is_focused: bool, rect: Rect) -> Self {
        Self { title, is_active, is_focused, rect }
    }
}