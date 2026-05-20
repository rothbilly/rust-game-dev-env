use macroquad::prelude::*;
use num_bigint::BigInt;

use crate::core::app::App;



#[derive(Default)]
pub struct Game{
    base_app: App,
    score: u32
}

impl Game {
    pub fn new(base_app: App, score: u32) -> Self {
        Self { base_app, score }
    }
    
    /// .Create a new game instance
    fn create() -> Self {
        Game::new(App::default(),  0)
    }
}


pub trait GameLoop{
    //fn _enter(){}
    fn _ready(&mut self) -> &mut Self;
    fn _update(&mut self, _delta: f32);
    //fn _draw(){}
    //fn _stop(){}
    //fn _exit(){}
    //fn _free(&mut self){}
}






#[derive(Default, PartialEq, Eq)]
pub struct GameProgressScore{
    score: BigInt,
    chips: BigInt,
    gems: BigInt

}

impl GameProgressScore {
    pub fn new(score: BigInt, chips: BigInt, gems: BigInt) -> Self {
        Self { score, chips, gems }
    }
}



#[cfg(test)]
mod tests {
    use std::io;
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_big_int(){
        //let game_dat = GameProgressScore::new(0, 0, 0);

        let mut string_data = String::new();

        println!("enter a custom amount A:");
        io::stdin().read_line(&mut string_data).expect("failed to read data");

        if string_data.parse::<u64>().is_ok(){

        }

        
        let value_a = BigInt::from_str(string_data.as_str()).expect("Failed to generate a bigit from string");

        println!("the result is {}", value_a);

        assert!(true, "pass the test");


    }
}