

use std::fmt::Debug;
use macroquad::prelude::*;

use crate::{core::game::GameLoop, editor::inspector::{InspectorWindow, app_loader::AppLoaderMenu}};


pub mod virtual_app;


/// # The App Base Module

#[derive(Default)]
pub struct App
{
    editor_visible: bool,
    sub_apps: Vec<Box<dyn GameLoop>>,
    inspectors: Vec<Box<dyn InspectorWindow>>
}

impl App
{
    pub fn new() -> Self {
        Self::default()
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
    where T:GameLoop + Default + 'static {
        self.sub_apps.push(Box::new(T::default()));
        if let Some(new_app) = self.sub_apps.last_mut(){
            new_app._ready();
        }
        self
    }

    pub fn add_sub_app(&mut self, new_app: Box<dyn GameLoop>) -> &mut Self
    {
        self.sub_apps.push(new_app);
        self
    }

    pub fn remove_last_app(&mut self){
        if self.sub_apps.is_empty(){
            return
        } else{
            let _ = self.sub_apps.pop();
            println!("Sub app removed from the app list")
        }
    }


    pub fn add_new_inspector<T>(&mut self)
    where T: InspectorWindow + Default + 'static
    {
        self.inspectors.push(Box::new(T::default()))
    }

    pub fn _ready(&mut self) -> &mut Self{
        for app in self.sub_apps.iter_mut(){
            app._ready();
        }
        self
    }

    /// the application process ordering
    pub fn _runtime(&mut self , delta_time: f32){
        if self.sub_apps.len() == 0 {return}

        for app in self.sub_apps.iter_mut(){
            app._process_input();
            app._update(delta_time);
            app._render();
        }

        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::RightAlt){
            self.editor_visible = !self.editor_visible
        }

        if !self.editor_visible{return}
        //Toggle Editor & Inspector Debug
        for inspector in self.inspectors.iter_mut(){

            //inspector.set_visible(!inspector.is_visible());

            if inspector.is_visible(){
                inspector._draw_gui();
                inspector._draw_egui();
            } 
        }

    }

    pub fn is_main_app(&self) -> bool{
        self.sub_apps.len() != 0 
    }
}


impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").field("editor_visible", &self.editor_visible).field("sub_apps", &self.sub_apps.len()).field("inspectors", &self.inspectors.len()).finish()
    }
}


