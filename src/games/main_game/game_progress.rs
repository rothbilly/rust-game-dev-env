
use num_bigint::BigInt;


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

        let string_data = String::from("1_000_000_000_000_000");

        println!("enter a custom amount A:");
        if string_data.parse::<u64>().is_ok(){

        }

        let value_a = BigInt::from_str(string_data.as_str()).expect("Failed to generate a bigit from string");

        println!("the result is {}", value_a);

        assert!(true, "pass the test");


    }
}