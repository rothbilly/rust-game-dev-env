use std::sync::{LazyLock, Mutex}; // For async.
use macroquad:: prelude::*;
use egui_macroquad::*;

//crate imports
use rust_game_dev_env as game_lib; 
use game_lib::core::game::GameLoop;
use game_lib::data::{settings::Settings, serialization::*};
use game_lib::mini_games::*;
/*
* static = eternal lifetime
* LazyLock = delays execution until it is called on code. Avoids having to define static at compile time.
* Mutex = Can only be edited under a lock. Avoids race conditions.
*/
static GLOB_SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Mutex::new(Settings::new()));

// This struct is from Macroquad and it's used to set parameters of the window.
fn app_conf() -> Conf {
    let settings = GLOB_SETTINGS.lock().unwrap().load_toml();

    Conf {
        window_title: "Dev Environment".to_owned(),
        window_width: settings.window_w,
        window_height: settings.window_h,
        window_resizable: true,
        ..Default::default()
    }
}

fn initialize_and_ready<T>() -> Box<T>
where
    T : GameLoop + Default + 'static
{
    let mut game = Box::new(T::default());
    game._ready();
    game
}

#[macroquad::main(app_conf)]
async fn main() {
    let mut game = initialize_and_ready::<MacrowareExe>();
    loop { // App loop
        let mut delta = get_frame_time();
        delta = delta.max(0.0).min(0.1);

        game._process_input();
        game._update(delta);
        game._render();


        if is_key_down(KeyCode::Escape) {
            on_close_application();
            break;
        }
        next_frame().await;
    }
}

fn on_close_application() {
    println!("[Application] - Closing!");
    GLOB_SETTINGS
        .lock()
        .unwrap()
        .save();
}

