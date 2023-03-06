extern crate rand;
use rand::Rng;

#[derive(PartialEq)]
pub enum CoinFlipResult {
    Heads,
    Tails,
}

pub fn flip_coin() -> CoinFlipResult {
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
