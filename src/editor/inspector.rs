use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use egui_macroquad::egui::{Context};
use macroquad::prelude::info;

use crate::editor::inspector;

pub mod event_inspector;
pub mod app_loader;


// Define a alias for a common composited type.
type InspectorItem =  Rc<RefCell<dyn InspectorWindow>>;

#[derive(Default)]
enum DrawingBackend{
    #[default]
    Macroquad,
    Egui
}

#[derive(Default)]
pub struct GlobalInspectors{
    pub editor_visible: bool,
    backend: DrawingBackend,
    inspectors: HashMap<String, InspectorItem>
}

impl GlobalInspectors{

    pub fn new_inspector<T>(&mut self)
    where T: InspectorWindow + Default + 'static
    {
        let new_inspector = Rc::new(RefCell::new(T::default()));
        let i_name = new_inspector.clone().borrow().get_struct_name();
        self.inspectors.insert(i_name,  new_inspector.clone());
    }

    pub fn add_inspector<T>(&mut self, glob_inspector: &Rc<RefCell<T>>)
    where T: InspectorWindow + Default + 'static
    {
        self.inspectors.insert(glob_inspector.borrow().get_struct_name(), glob_inspector.clone()); // pass a reference to a currently existing inspector.

    }

    pub fn  toggle_global_visibility(&mut self){
        self.editor_visible = !self.editor_visible;
        info!("editors counters {}", self.inspectors.len())
    }

    pub fn _editor_runtime(&self) {
        if self.editor_visible
        {
            match self.backend {
                DrawingBackend::Macroquad => {
                    for (_, inspector) in self.inspectors.iter(){
                        inspector.borrow_mut()._draw_gui();
                    }
                }

                DrawingBackend::Egui =>{
                    egui_macroquad::ui(|egui_ctx| {
                        for (_,inspector) in self.inspectors.iter(){
                            inspector.borrow_mut()._draw_egui(&egui_ctx);
                        }
                    });
                    egui_macroquad::draw();
                }
            }
        }
    }
}

pub trait InspectorWindow: IVisibility + IName + Inspector{}


pub trait IName{
    fn get_struct_name(&self) -> String {
        stringify!(self).to_string() //
    }
}

pub trait IVisibility{
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, _value: bool);
}

// Draw the GUI
pub trait Inspector {

    fn _setup(&mut self){
    }
    fn _draw_gui(&mut self) ->bool{
        false
    }
    fn _draw_egui(&self, _egui_ctx: &Context){

    }
}
