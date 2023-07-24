use std::io;
use std::thread;
use std::time::Duration;

// Criar um programa que roda eternamente esperando input dos usuários, e processa em threads as ações

pub fn exec(){
    loop {
        println!("Enter your input: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let duration = Duration::from_secs(5);
    
        println!("Starting timer...");
        
        thread::sleep(duration);
        
        println!("You entered: {}", input.trim());
        
        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }
    }
}
