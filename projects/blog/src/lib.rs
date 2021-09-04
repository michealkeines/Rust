pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approval: u32
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approval: 0
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_content(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            let tmp = s.update();
            if tmp.0 == true {
                self.content.push_str(text);
                self.state = Some(tmp.1);
            } else {
                let t = "";
                self.content.push_str(&t);
                self.state = Some(tmp.1);
            }
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(self))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn update(self: Box<Self>) -> (bool,Box<dyn State>);
}

struct Draft {

}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn update(self: Box<Self>) -> (bool,Box<dyn State>) {
        (true, self)
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {

        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {

}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn update(self: Box<Self>) -> (bool,Box<dyn State>) {
        (false, self)
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        if post.approval < 1 {
            post.approval = 1;
            self
        } else {
            post.approval = 0;
            Box::new(Published {})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {

}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn update(self: Box<Self>) -> (bool,Box<dyn State>) {
        (false, self)
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_new_post() {
    /*

        A blog post starts as an empty draft.
        When the draft is done, a review of the post is requested.
        When the post is approved, it gets published.
        Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

        Add a reject method that changes the post’s state from PendingReview back to Draft.
        Require two calls to approve before the state can be changed to Published.
        Allow users to add text content only when a post is in the Draft state.

    */

        let mut a = Post::new();

        assert_eq!(a.content(),""); // intial emtpy string check

        let s = String::from("This is the text");

        a.add_content(&s); // adding new content to the draft state
        assert_eq!(a.content,"This is the text"); // check if the text is added by checking the private feild
        assert_eq!(a.content(),""); // call get content this will still return nothing as we are in draft state

        a.request_review(); // we request for an review returns pending review
        assert_eq!(a.content(),""); // still the content will be nothing
        
        a.reject(); // reject the review
        assert_eq!(a.content(), ""); // content will be nothing
        
        a.request_review(); // again request for an review
        a.approve(); // call approve once
        assert_eq!(a.content(), ""); // now it returns the nothing as only one appoval

        let k = String::from("This is the textssdfsdf");

        a.add_content(&k); // try to update content in review pending state
        assert_eq!(a.content(),"");      

        a.reject(); // reject the review
        assert_eq!(a.content(), ""); // content will be nothing

        a.request_review(); // again request for an review
        a.approve(); // call aprove twice to get published
        a.approve(); // call aprove twice to get published
        assert_eq!(a.content(), "This is the text"); // now it returns the stirng
   
        let u = String::from("This is the textssdfsdf");

        a.add_content(&u); // try to update content in published state

        assert_eq!(a.content(), "This is the text");
    }
}
