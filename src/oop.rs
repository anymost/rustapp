pub mod post {
    pub struct Post {
        content: String,
        state: Option<Box<State>>,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                content: String::new(),
                state: Some(Box::new(Draft{}))
            }
        }

        pub fn add_text(&mut self, v: &str) {
            self.content.push_str(v);
        }

        pub fn content(&self) -> &str {
            ""
        }
    }

    trait State {}

    struct Draft {}

    impl State for Draft {}
}

fn main() {

}