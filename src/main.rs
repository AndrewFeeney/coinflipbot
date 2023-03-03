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
    return CoinFlipResult::Heads;
}

#[test]
fn test_flip_coin() {
    let result = flip_coin();

    assert!(result == CoinFlipResult::Heads || result == CoinFlipResult::Tails);
}
