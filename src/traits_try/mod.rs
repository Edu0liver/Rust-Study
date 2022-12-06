
mod traits_try{
    use std::fmt::Display;
    use std::io;

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
        let mut input = String::new();
        let mut input2 = String::new();

        println!("Enter a number: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        println!("Enter another number: ");
        io::stdin().read_line(&mut input2).expect("Failed to read line");

        let input: i32 = input.trim().parse().expect("Please type a number!");
        let input2: i32 = input2.trim().parse().expect("Please type a number!");
        
        let pair = Pair::new(input, input2);
        pair.cmp_display();
        println!("Pair: {}", pair.show());
    }

}

pub fn traits_try() {
    crate::traits_try::traits_try::traits_call();
}
