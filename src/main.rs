

use std::rc::Rc;
use std::sync::{LazyLock, Mutex, RwLock}; 


// For async.
use macroquad:: prelude::*;


// egui
use egui_macroquad::*;
use puffin;



// own crate imports
use rust_game_dev_env as game_lib; 
use game_lib::core::signal::Signal;
use game_lib::editor::inspector::Inspector;
use game_lib::editor::inspector::app_loader::AppLoaderMenu;
use game_lib::games::{MacrowareExe, TestGame};
use game_lib::core::{app::*};
use game_lib::data::{settings::Settings};
use game_lib::editor::inspector::event_inspector::*;
use game_lib::core::app::virtual_app::SampleVirtualApp;



/// static = eternal lifetime
/// LazyLock = delays execution until it is called on code. Avoids having to define static at compile time.
/// Mutex = Can only be edited under a lock. Avoids race conditions.
static GLOB_SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Mutex::new(Settings::new()));


// This struct is from Macroquad and it's used to set parameters of the window.
fn app_conf() -> Conf {
    let settings = GLOB_SETTINGS.lock().unwrap().load_toml();

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
    let main_app = Rc::new(RwLock::new(App::initialize_and_ready::<SampleVirtualApp>())); // Rc count 1
    

    // Editor: move it to other part in the future.
    let mut ev = EventInspector::default();
    let mut app_loader = AppLoaderMenu::default();
    



    let _app = main_app.clone(); // Rc count 2 -- app_clone is moved inside the closure because of `move`
    app_loader.on_click_signal.connect( move |_option| {
        

        // move this to internal method in editor
        match _option {
            Signal::IntCallback(_value) =>{
                
                match _value {
                    0 => { _app.write().unwrap().load_and_replace_app_at_last::<SampleVirtualApp>();},
                    1 => { _app.write().unwrap().load_and_replace_app_at_last::<MacrowareExe>(); },
                    2 => { _app.write().unwrap().load_and_replace_app_at_last::<TestGame>();},
                    _ => ()
                }
                _app.read().unwrap().get_total_apps();
            }
            _ => ()
        }
    });
    
    let app_ref = main_app.clone(); // Rc count 3
    let mut editor_visible: bool = false;

    loop { // Main loop
        let mut delta = get_frame_time();
        delta = delta.max(0.0).min(0.1);


        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::LeftAlt){
            editor_visible = !editor_visible;
        }
        if editor_visible{
            ev._draw_gui();
            app_loader._draw_gui();
        }

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
    let _ = GLOB_SETTINGS        .lock()
        .unwrap()
        .save();

    

}

