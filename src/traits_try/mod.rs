
mod traits_try{
    use std::fmt::Display; 

    struct Pair<T> {
        x: T,
        y: T
    }

    impl <T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    trait Show {
        fn show(&self) -> String;
    }

    impl<T: Display> Show for Pair<T> {
        fn show(&self) -> String {
            format!("x: {}, y: {}", self.x, self.y)
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("Pair: The largest member is x = {}", self.x);
            } else {
                println!("Pair: The largest member is y = {}", self.y);
            }
        }
    }

    pub fn traits_call() {
        let pair = Pair::new(2, 3);
        pair.cmp_display();
        
        let pair2 = Pair::new(2, 3);
        println!("Pair2: {}", pair2.show());
    }

}

pub fn traits_try() {
    crate::traits_try::traits_try::traits_call();
}
