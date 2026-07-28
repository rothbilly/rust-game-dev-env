

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use macroquad::prelude::*;


use crate::core::game::{ GameLoop};
use std::collections::HashMap;


// Define a alias for a common composited type.
type AppConstructor = fn() -> Rc<RefCell<dyn GameLoop>>; // Every app implements a GameLoop.

// ==========================================
//  AppRegistration
// ==========================================

pub struct AppRegistration {
    pub name : &'static str,
    pub constructor : fn() -> Rc<RefCell<dyn GameLoop>>,
}

inventory::collect!(AppRegistration);

#[derive(Debug)]
pub struct AppRegistry {
    pub registry : HashMap<& 'static str, AppConstructor>,
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AppRegistry{
    pub fn new() -> Self{
        let mut r = Self { registry : HashMap::new() };

        for app in inventory::iter::<AppRegistration> {
            r.registry.insert(app.name, app.constructor);
            info!("[Registry] - Automatically registered app: {}", app.name);
        }
        r
    }

    

    fn register<T>(&mut self, tag: &'static str)
    where T: GameLoop + Default + 'static {
        self.registry.insert(tag, || Rc::new(RefCell::new(T::default())));
    }

    pub fn get(&self, tag: &str) -> Option<Rc<RefCell<dyn GameLoop>>> {
        self.registry.get(tag).map(|constructor| constructor())
    }

}

/// ==========================================
///  App List Module
/// ==========================================
/// The list of active apps currently running or loaded.
/// for now it can be one already.


#[derive(Default)]
pub struct AppList
{
    is_in_debug_mode: bool,
    active_apps: Vec<Rc<RefCell<dyn GameLoop>>>,
}

impl AppList
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize_and_ready<T>() -> AppList
    where T:GameLoop + Default + 'static
    {
        let mut app = AppList::default();
        app.active_apps.push(Rc::new(RefCell::new(T::default())));
        app._ready();
        app
    }


    pub fn append_new_sub_app(&mut self, shared_app : Rc<RefCell<dyn GameLoop>>) -> &mut Self{

        self.active_apps.push(shared_app); // We cannot create an RC RefCell because of conflict with <dyn GameLoop>

        if let Some(new_app) = self.active_apps.last_mut(){
            new_app.clone()
            .borrow_mut()
            .ready()
        }
        info!("[Main App] - Added a new sub app successfully");
        self
    }


    pub fn load_and_replace_app_at_last(&mut self, new_app : Rc<RefCell<dyn GameLoop>>) -> &mut Self{
        self.remove_last_app();
        self.append_new_sub_app(new_app);
        self
    }


        pub fn new_app_as<T>(&mut self) -> &mut Self
    where T:GameLoop + Default + 'static
    {
        let wrapper = Rc::new(RefCell::new(T::default()));
        self.active_apps.push(wrapper);
        self
    }


    pub fn add_sub_app<T>(&mut self, new_app: T) -> &mut Self
    where T:GameLoop + Default + 'static
    {
        let wrapper = Rc::new(RefCell::new(new_app));
        self.active_apps.push(wrapper);
        self
    }

    pub fn remove_last_app(&mut self){
        if self.active_apps.is_empty(){
        } else{
            //Deref a sub app from the vec! and drop its pointer from te Rc.
            //if there a other pointer to the sub app, it keeps in memory until the last Rc is deref.
            let removed_app = self.active_apps.pop();
            if let Some(app) = removed_app{
                app.clone().borrow_mut()._cleanup();
            }
            println!("[Main App] - App removed from the Sub app list")
        }
    }

    pub fn get_total_apps(&self ){
        println!("[Main App] - total of sub app: {}", self.active_apps.len());
    }

    pub fn is_main_app(&self) -> bool{
        !self.active_apps.is_empty() 
    }

    pub fn toggle_debug_view(&mut self){
        self.is_in_debug_mode = !self.is_in_debug_mode
    }


    // ==========================================
    //  App Module Built In Calls
    // ==========================================

    pub fn _ready(&self) -> &Self{
        for app in self.active_apps.iter(){
            app.clone()
            .borrow_mut()
            .ready();
        }
        self
    }

    
    pub fn _runtime(&self , delta_time: f32){
        if self.active_apps.is_empty() {return}

        for rc in self.active_apps.iter(){
            let _rc = rc.clone();
            let mut _app = _rc.borrow_mut();
            _app.process_input();
            _app.update(delta_time);
            _app.render();
            if self.is_in_debug_mode{
                _app._debug_draw();
            }
        }

    }


}


impl Debug for AppList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").field("sub_apps", &self.active_apps.len()).finish()
    }
}


