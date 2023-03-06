extern crate rand;
use rand::Rng;

fn main() {
    let result = flip_coin();

    match result {
        CoinFlipResult::Heads => println!("The result was heads!"),
        CoinFlipResult::Tails => println!("The result was tails!"),
    }
}

struct SocialMediaPost {
    text: String,
}
struct SocialMediaPostResult {
    success: bool,
    post: SocialMediaPost,
}

struct TestMastodonAccount {
    posts: Vec<String>,
}

trait Postable {
    fn post(&mut self, text: String) -> SocialMediaPostResult;
}

impl Postable for TestMastodonAccount {
    fn post(&mut self, text: String) -> SocialMediaPostResult {
        self.posts.push(text.clone());

        SocialMediaPostResult {
            success: true,
            post: SocialMediaPost {
                text,
            }
        }
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

#[test]
fn test_post() {
    let mut test_account = TestMastodonAccount {
        posts: Vec::new(),
    };

    let text = "Hello, world!".to_string();
    let post_result = test_account.post(text);

    assert!(post_result.success == true);
    assert!(post_result.post.text == "Hello, world!");
}
