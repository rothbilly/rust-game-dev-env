use macroquad::prelude::*;


#[derive(Default)]
pub enum WindowMode {
    #[default]
    Mode2D,
    Mode3D,
    ModeEditor
}


#[derive(Default)]
pub struct VirtualWindow{
    title: String,
    mode: WindowMode,
    is_active: bool,
    is_focused: bool,
    rect: Rect
}



