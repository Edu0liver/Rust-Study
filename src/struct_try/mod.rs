
mod struct_try {
    use std::io;

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8
    }

    impl Person {
        fn new(name: String, age: u8) -> Person {
            Person {
                name: name_format(name),
                age
            }
        }
    }

    fn name_upper(word: String) -> String {
        let mut vec: Vec<char> = word.chars().collect();

        vec[0] = vec[0].to_uppercase().nth(0).unwrap();

        let word: String = vec.into_iter().collect();

        word
    }

    fn split_name(name: &String) -> Vec<String> {
        let vec_str: Vec<&str> = name.split(" ").collect();
        let mut vec_string: Vec<String> = Vec::new();

        for name in vec_str.iter() {
            vec_string.push(name.to_string());
        }

        vec_string
    }

    fn name_title_format(names: &Vec<String>) -> Vec<String> {
        let mut vec = Vec::new();

        for name in names.iter() {
            vec.push(name_upper(name.to_string()))
        }
        
        vec
    }

    fn name_format(name: String) -> String {
        let names = split_name(&name);

        let name_formated = name_title_format(&names);

        name_formated.join(" ")
    }

    pub fn struct_call() {
        let mut input_name = String::new();
        let mut input_age = String::new();

        println!("Enter your name: ");
        io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read line");

        let name = input_name
        .trim()
        .to_string();

        println!("Enter your age: ");
        io::stdin()
        .read_line(&mut input_age)
        .expect("Failed to read line");

        let age: u8 = input_age
        .trim()
        .parse()
        .expect("Failed to parse");

        let person = Person::new(name, age);
    
        println!("{:#?}", person);
    }

}

pub fn struct_try() {
    crate::struct_try::struct_try::struct_call();
}
