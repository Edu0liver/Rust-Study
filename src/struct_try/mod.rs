
mod struct_try {
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
        let mut vec = Vec::new();

        name.split(" ").for_each(|s| vec.push(s.to_string()));
        
        vec
    }

    fn name_title_format(names: &Vec<String>) -> Vec<String> {
        let mut vec = Vec::new();

        names.iter().for_each(|s| vec.push(name_upper(s.to_string())));
        
        vec
    }

    fn name_format(name: String) -> String {
        let names = split_name(&name);

        let name_formated = name_title_format(&names);

        name_formated.join(" ")
    }

    pub fn struct_call() {
        let person = Person::new(String::from("eduardo alves"), 19);
    
        println!("{:#?}", person);
    }

}

pub fn struct_try() {
    crate::struct_try::struct_try::struct_call();
}
