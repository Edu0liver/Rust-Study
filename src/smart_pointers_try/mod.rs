
mod smart_pointers_try {
    use std::ops::Deref;
    use std::rc::Rc;

    //Box smart pointer
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    //Deref coercion
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    //Drop trait
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    //Rc smart pointer
    enum ListRc {
        Cons(i32, Rc<ListRc>),
        Nil,
    }

    //Interior mutability
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: 'a + Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    use crate::smart_pointers_try::smart_pointers_try::ListRc::{Cons, Nil};

    pub fn smart_pointers_call() {
        //Box smart pointer
        println!("\nBox smart pointer");
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
        println!("List: {:?}", list);

        //Deref coercion
        println!("\nDeref coercion");
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *(y.deref()));

        //Deref coercion with custom types
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);

        //Drop trait
        println!("\nDrop trait");
        let c = CustomSmartPointer { data: String::from("my stuff") };
        let d = CustomSmartPointer { data: String::from("other stuff") };
        //Don't need to call drop explicitly
        //But if we do, it will be called as a regular function
        drop(c);
        drop(d);
        println!("CustomSmartPointers created.");

        //Rc smart pointer
        println!("\nRc smart pointer");
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
        //Increasing the reference count
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

}

pub fn smart_pointers_try() {
    crate::smart_pointers_try::smart_pointers_try::smart_pointers_call();
}

//Interior mutability
mod tests {
    use super::*;
    use std::cell::RefCell;
    use crate::smart_pointers_try::smart_pointers_try::{Messenger, LimitTracker};

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
