
use macroquad::{color, prelude::*};
use crate::core::game::{GameLoop};


#[derive(Default)]
pub struct SampleVirtualApp{
    title: String,
    mode: VirtualAppMode,
    rect: Rect,
    is_active: bool,
    base_app: Option<Box<dyn IParentable>>
}


/// # App Mode 
/// Define if the app is used for 2D or 3D Rendering or as editor UI
#[derive(Default)]
pub enum VirtualAppMode {
    #[default]
    Mode2D,
    Mode3D,
    ModeUI,
    Headless
}


impl GameLoop for SampleVirtualApp {
    fn _render(&mut self) {
        clear_background(color::LIGHTGRAY);

        draw_text(&format!("FPS: {}", get_fps()), 16., 24., 24., color::RED);
        
        let center = get_text_center("Macro Engine", None, 32, 1.0, 0.);
        
        let td = draw_text("Macro Engine",
        screen_width()/2. - center.x,
        screen_height()/2. - center.y,
        32.,
        color::GRAY);
    
    }
}
impl IParentable for SampleVirtualApp {
    fn get_base_app(&self) -> Option<&Box<dyn IParentable>> {
        self.base_app.as_ref()
    }
}
impl IPausable for SampleVirtualApp{
    fn is_active(&self) -> bool{
        self.is_active
    }
    fn set_active(&mut self, _value: bool){
       self.is_active = _value 
    }
}

// Example: In this moment only works as a tag for some structs, for example get all struct that implement this.
impl IGame for SampleVirtualApp{}
impl IVirtualApp for SampleVirtualApp{}

/// This can be converted in a derive macro as #[derive(IVirtualApp)] that implement the sub traits
pub trait IGame : GameLoop + IParentable + IPausable{}
pub trait IVirtualApp : IParentable + IPausable{}



/// Enforce the struct to have a composition base_app field; 
pub trait IParentable{
    fn get_base_app(&self) -> Option<&Box<dyn IParentable>>;
}

pub trait IPausable {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, _value: bool);
}
