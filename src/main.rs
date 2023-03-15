mod coin;
mod social;
mod http;
use social::Postable;

fn main() {
    let result = coin::flip();

    let post_text;

    match result {
        coin::CoinFlipResult::Heads => post_text = "The result was heads!",
        coin::CoinFlipResult::Tails => post_text = "The result was tails!",
    }
 
    let mut test_account = social::TestMastodonAccount {
        posts: Vec::new(),
    };

    let post_result = test_account.post(post_text.to_string());

    if post_result.success {
        println!("{:?}", post_result.post.text);
    } else {
        println!("The post failed")
    }
}
