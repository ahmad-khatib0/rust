pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // takes a mutable reference to self because we’re changing the Post
    // instance that we’re calling add_text on.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // self.state.take:  lets us move the state value out of Post rather than borrowing it.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // self: Box<Self>. This syntax means the method is only valid when called on a Box
    // holding the type. This syntax takes ownership of Box<Self>, invalidating the old
    // state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // we don’t need to implement content on the Draft and PendingReview structs.
    // We’re taking a reference to a post as an argument and returning a reference to part of that
    // post, so the lifetime of the returned reference is related to the lifetime of the post argument.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct PendingReview {}
struct Draft {}
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    // it returns itself  because when we request a review on a post already
    // in the PendingReview state, it should stay in the PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
