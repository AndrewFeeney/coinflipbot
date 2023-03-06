mod coin;

fn main() {
    let result = coin::flip();

    match result {
        coin::CoinFlipResult::Heads => println!("The result was heads!"),
        coin::CoinFlipResult::Tails => println!("The result was tails!"),
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
