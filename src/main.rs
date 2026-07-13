
use macroquad:: prelude::*;
use rust_game_dev_env::core::engine::{Engine, app_config};
use rust_game_dev_env::data::settings::{Settings, GLOB_SETTINGS};


#[macroquad::main(app_config)]
async fn main() {

    let mut engine = Engine::new();
    engine.setup();


    loop { // Main loop
        engine.engine_loop();
        next_frame().await;
    }
}


