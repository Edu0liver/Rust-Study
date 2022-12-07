
mod smart_pointers_try {
    use std::ops::Deref;

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

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

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    pub fn smart_pointers_call() {
        //Box smart pointer
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
        println!("List: {:#?}", list);

        //Deref coercion
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *(y.deref()));

        //Deref coercion with custom types
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);

        //Drop trait
        let c = CustomSmartPointer { data: String::from("my stuff") };
        let d = CustomSmartPointer { data: String::from("other stuff") };
        //Don't need to call drop explicitly
        //But if we do, it will be called as a regular function
        drop(c);
        drop(d);
        println!("CustomSmartPointers created.");
    }

}

pub fn smart_pointers_try() {
    crate::smart_pointers_try::smart_pointers_try::smart_pointers_call();
}
