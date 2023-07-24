
pub mod choice {
    
    use std::io;

    use crate::guessing_game::game;
    use crate::struct_try::struct_try;
    use crate::enum_try::enum_try;
    use crate::generic_try::generic_try;
    use crate::traits_try::traits_try;
    use crate::smart_pointers_try::smart_pointers_try;
    use crate::threads_try::threads_try;
    use crate::oop_try::oop_try;
    use crate::user_manage::user_manage;
    use crate::tabuada::exec;
    use crate::elevator::exec as elevator_exec;
    
    pub fn choice(){
        println!("What do you want to try? Tip a number:");
        println!("1. Guessing Game");
        println!("2. Struct");
        println!("3. Enum");
        println!("4. Generic");
        println!("5. Traits");
        println!("6. Smart Pointers");
        println!("7. Threads");
        println!("8. OOP");
        println!("9. User Manage");
        println!("10. Tabuada");
        println!("11. Elevator Program");
        println!("12. Exit");
    
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
            "6" => smart_pointers_try(),
            "7" => threads_try(),
            "8" => oop_try(),
            "9" => user_manage(),
            "10" => exec(),
            "11" => elevator_exec(),
            "12" => (),
            _ => println!("Invalid option"),
        }
    }
}
