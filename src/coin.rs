extern crate rand;
use rand::Rng;

#[derive(PartialEq)]
pub enum CoinFlipResult {
    Heads,
    Tails,
}

pub fn flip() -> CoinFlipResult {
    let mut random_number_generator = rand::thread_rng();
    if random_number_generator.gen::<bool>() {
        return CoinFlipResult::Heads
    }

    CoinFlipResult::Tails
}

#[test]
fn test_flip() {
    let result = flip();

    assert!(result == CoinFlipResult::Heads || result == CoinFlipResult::Tails);
}
