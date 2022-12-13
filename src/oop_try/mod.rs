
mod oop_try {

    fn ex_1(){

        struct AvaregedCollection {
            list: Vec<i32>,
            average: f64,
        }

        impl AvaregedCollection {
            fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    },
                    None => None,
                }
            }

            fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }

    }

    //////////////////////////////////////// Ex 2
    pub trait Draw {
        fn draw(&self);
    }
    
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Draw a button")
        }
    }

    /////////////////////////////////////// Ex 3
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }
    
    struct Draft {}
    
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    
    struct PendingReview {}
    
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

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

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

    }

    pub fn oop_call() {
        ex_1()
    }

}

pub mod ex2_oop_try {
    use crate::oop_try::oop_try::{Draw, Button, Screen};

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Draw a select box")
        }
    }

    pub fn ex2_oop_call() {
        let screen = Screen {
            components: vec![
                Box::new(
                    SelectBox {
                        width: 75,
                        height: 10,
                        options: vec![
                            String::from("Yes"),
                            String::from("Maybe"),
                            String::from("No"),
                        ],
                    }
                ),
                Box::new(
                    Button {
                        width: 50,
                        height: 10,
                        label: String::from("OK"),
                    }
                ),
            ],
        };
    
        screen.run();
    }
}
pub mod ex3_oop_try {
    use crate::oop_try::oop_try::Post;

    pub fn ex3_oop_call() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

}

pub fn oop_try() {
    crate::oop_try::oop_try::oop_call();
    crate::oop_try::ex2_oop_try::ex2_oop_call();
}
