
use macroquad::prelude::*;
use crate::core::game::{GameLoop};




#[derive(Default)]
pub struct SampleVirtualApp{
    is_active: bool,
    base_app: Option<Box<dyn IParentable>>

}


impl GameLoop for SampleVirtualApp {}
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



pub trait IGame : IVirtualApp + GameLoop{}
trait IVirtualApp : IParentable + IPausable{}



/// Enforce the struct to have a composition base_app field; 
pub trait IParentable{
    fn get_base_app(&self) -> Option<&Box<dyn IParentable>>;
}

pub trait IPausable {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, _value: bool);
}
