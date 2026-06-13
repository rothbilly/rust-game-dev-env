

use std::{cell::RefCell, fmt::Debug, rc::Rc};
use macroquad::prelude::*;


use crate::editor::inspector::app_loader::AppType;
use crate::core::game::GameLoop;
use crate::core::app::virtual_app::SampleVirtualApp;
use crate::games::MacrowareExe;
use crate::games::TestGame;


pub mod virtual_app;


/// # The App Base Module
#[derive(Default)]
pub struct App
{
    sub_apps: Vec<Rc<RefCell<dyn GameLoop>>>,
}

impl App
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_by_type(&mut self, app_type : AppType) -> &mut App{
        match app_type {
            AppType::SampleVirtualApp => self.load_and_replace_app_at_last::<SampleVirtualApp>(),
            AppType::MacroExe => self.load_and_replace_app_at_last::<MacrowareExe>(),
            AppType::Breakout => self.load_and_replace_app_at_last::<TestGame>(),
        }
    }

    pub fn initialize_and_ready<T>() -> App
    where T:GameLoop + Default + 'static
    {
        let mut app = App::default();
        app.sub_apps.push(Rc::new(RefCell::new(T::default())));
        app._ready();
        app
    }

    pub fn append_new_sub_app<T>(&mut self) -> &mut Self
    where T:GameLoop + Default + 'static {
        self.sub_apps.push(Rc::new(RefCell::new(T::default())));
        puffin::profile_function!();
        if let Some(new_app) = self.sub_apps.last_mut(){
            new_app.clone()
            .borrow_mut()
            ._ready();
        }
        println!("[Main App] - Added a new sub app successfully");
        self
    }

    pub fn load_and_replace_app_at_last<T>(&mut self) -> &mut Self
    where T:GameLoop + Default + 'static {
        self.remove_last_app();
        self.append_new_sub_app::<T>();
        self
    }

    pub fn add_sub_app<T>(&mut self, new_app: T) -> &mut Self
    where T:GameLoop + Default + 'static
    {
        let wrapper = Rc::new(RefCell::new(new_app));
        self.sub_apps.push(wrapper);
        self
    }

    pub fn remove_last_app(&mut self){
        if self.sub_apps.is_empty(){
            return
        } else{
            let _ = self.sub_apps.pop();
            println!("[Main App] - App removed from the Sub app list")
        }
    }

    pub fn get_total_apps(&self ){
        println!("[Main App] - total of sub app: {}", self.sub_apps.len());
    }


    pub fn _ready(&self) -> &Self{
        for app in self.sub_apps.iter(){
            app.clone()
            .borrow_mut()
            ._ready();
        }
        self
    }

    /// the application process ordering
    pub fn _runtime(&self , delta_time: f32){
        if self.sub_apps.len() == 0 {return}

        for rc in self.sub_apps.iter(){
            let _rc = rc.clone();
            let mut _app = _rc.borrow_mut();
            _app._process_input();
            _app._update(delta_time);
            _app._render();
        }

    }

    pub fn is_main_app(&self) -> bool{
        self.sub_apps.len() != 0 
    }
}


impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").field("sub_apps", &self.sub_apps.len()).finish()
    }
}


