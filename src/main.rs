

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock}; 


// For async.
use macroquad:: prelude::*;
// egui
use egui_macroquad::*;


use rust_game_dev_env::editor::inspector::GlobalInspectors;
// own crate imports
use rust_game_dev_env as game_lib; 
use game_lib::core::signal::Signal;
use game_lib::editor::inspector::Inspector;
use game_lib::editor::inspector::app_loader::AppLoaderMenu;
use game_lib::core::{app::*};
use game_lib::data::{settings::Settings};
use game_lib::editor::inspector::event_inspector::*;
use game_lib::games::virtual_app::SampleVirtualApp;


/// static = eternal lifetime
/// LazyLock = delays execution until it is called on code. Avoids having to define static at compile time.
/// Mutex = Can only be edited under a lock. Avoids race conditions.
static GLOB_SETTINGS: LazyLock<RwLock<Settings>> = LazyLock::new(|| RwLock::new(Settings::new()));


// This struct is from Macroquad and it's used to set parameters of the window.
fn app_conf() -> Conf {
    let settings = GLOB_SETTINGS.write().unwrap().load_toml();
    
    Conf {
        window_title: "MacroQuack".to_owned(),
        window_width: settings.window_w,
        window_height: settings.window_h,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(app_conf)]
async fn main() {

    // If Non Static - Reference Counted Rc, Interior Mutability by a RefCell/RwLock -- Maybe in the future use Arc<RwLock>
    let main_app: Rc<RwLock<App>> = Rc::new(RwLock::new(App::initialize_and_ready::<SampleVirtualApp>())); 
    let global_inspectors: Rc<RwLock<GlobalInspectors>> = Rc::new(RwLock::new(GlobalInspectors::default()));


    let app_loader = Rc::new(RefCell::new(AppLoaderMenu::default()));

    global_inspectors.write().unwrap().new_inspector::<EventInspector>();
    global_inspectors.write().unwrap().add_inspector::<AppLoaderMenu>(&app_loader);

    let _app = main_app.clone(); // Rc count 2 -- app_clone is moved inside the closure because of `move`
    let app_registry = app_loader.borrow().app_registry.clone();


    app_loader.borrow_mut().on_click_signal.connect(move |signal| {
        if let Signal::LoadApp(tag) = signal {
            if let Some(new_app) = app_registry.get(tag){
                let mut app_write = _app.write().unwrap();

                app_write.load_and_replace_app_at_last_2(new_app);
                //app_write.get_total_apps();
            }else{
                println!("[Editor] Error: Tag {} not in registry.", tag);
            }
        }
    });
    
    let app_ref = main_app.clone(); // Rc count 3
    app_loader.borrow_mut()._setup();

    loop { // Main loop
        let mut delta = get_frame_time();
        delta = delta.max(0.0).min(0.1);

        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::LeftAlt){
            global_inspectors.write().unwrap().toggle_global_visibility()
        }

        global_inspectors.read().unwrap()._editor_runtime();
        app_ref.read().unwrap()._runtime(delta);

        if is_key_down(KeyCode::Escape) {
            on_close_application();
            break;
        }

        next_frame().await;
    }
}

fn on_close_application() {


    

    println!("[Application] - Closing!");
    let _ = GLOB_SETTINGS.read()
        .unwrap()
        .save();
}

