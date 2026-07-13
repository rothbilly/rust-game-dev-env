
use macroquad:: prelude::*;
use rust_game_dev_env::core::engine::{Engine, app_config};


#[macroquad::main(app_config)]
async fn main() {

    let mut engine = Engine::new();
    engine.setup()
    .with_starting_app();


    loop { // Main loop
        engine.engine_loop();
        next_frame().await;
    }
}


