

use std::{cell::RefCell, fmt::Debug, rc::Rc};
use macroquad::prelude::*;


use crate::core::game::GameLoop;
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

pub struct AppRegistry {
    pub registry : HashMap<& 'static str, AppConstructor>,
}

impl AppRegistry{
    pub fn new() -> Self{
        let mut r = Self { registry : HashMap::new() };

        for app in inventory::iter::<AppRegistration> {
            r.registry.insert(app.name, app.constructor);
            println!("[Registry] - Automatically registered app: {}", app.name);
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

// ==========================================
//  App Module
// ==========================================


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

    pub fn append_new_sub_app_2(&mut self, shared_app : Rc<RefCell<dyn GameLoop>>) -> &mut Self{

        self.sub_apps.push(shared_app); // We cannot create an RC RefCell because of conflict with <dyn GameLoop>

        if let Some(new_app) = self.sub_apps.last_mut(){
            new_app.clone()
            .borrow_mut()
            ._ready()
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

    pub fn load_and_replace_app_at_last_2(&mut self, new_app : Rc<RefCell<dyn GameLoop>>) -> &mut Self{
        self.remove_last_app();
        self.append_new_sub_app_2(new_app);
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
            //Deref a sub app from the vec! and drop its pointer from te Rc.
            //if there a other pointer to the sub app, it keeps in memory until the last Rc is deref.
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


