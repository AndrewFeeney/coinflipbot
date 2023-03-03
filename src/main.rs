fn main() {
    let result = flip_coin();

    match result {
        CoinFlipResult::Heads => println!("The result was heads!"),
        CoinFlipResult::Tails => println!("The result was tails!"),
    }
}

enum CoinFlipResult {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlipResult {
    return CoinFlipResult::Heads;
}
