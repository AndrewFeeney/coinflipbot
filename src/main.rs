extern crate rand;
use rand::Rng;

fn main() {
    let result = flip_coin();

    match result {
        CoinFlipResult::Heads => println!("The result was heads!"),
        CoinFlipResult::Tails => println!("The result was tails!"),
    }
}

#[derive(PartialEq)]
enum CoinFlipResult {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlipResult {
    let mut random_number_generator = rand::thread_rng();
    if random_number_generator.gen::<bool>() {
        return CoinFlipResult::Heads
    }

    CoinFlipResult::Tails
}

#[test]
fn test_flip_coin() {
    let result = flip_coin();

    assert!(result == CoinFlipResult::Heads || result == CoinFlipResult::Tails);
}
