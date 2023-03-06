pub struct SocialMediaPost {
    pub text: String,
}

pub struct SocialMediaPostResult {
    pub success: bool,
    pub post: SocialMediaPost,
}

pub trait Postable {
    fn post(&mut self, text: String) -> SocialMediaPostResult;
}

pub struct TestMastodonAccount {
    pub posts: Vec<String>,
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

