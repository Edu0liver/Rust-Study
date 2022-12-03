
pub mod choice {
    
    use std::io;

    use crate::guessing_game::game;
    use crate::struct_try::struct_try;
    use crate::enum_try::enum_try;
    use crate::generic_try::generic_try;
    use crate::traits_try::traits_try;
    
    pub fn choice(){
        println!("What do you want to try? Tip a number:");
        println!("1. Guessing Game");
        println!("2. Struct");
        println!("3. Enum");
        println!("4. Generic");
        println!("5. Traits");
        println!("6. Exit");
    
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .unwrap();
    
        let input = input.trim().to_string();
    
        match input.as_str() {
            "1" => game(),
            "2" => struct_try(),
            "3" => enum_try(),
            "4" => generic_try(),
            "5" => traits_try(),
            "6" => (),
            _ => println!("Invalid option"),
        }
    }
}
