
mod user_manage {
    use uuid::Uuid;

    #[derive(Debug)]
    struct User<'a> {
        id: Uuid,
        name: &'a str,
        age: u8,
        email: &'a str,
        password: &'a str,
        admin: bool
    }

    impl User<'_> {
        fn new(name: &'static str, age: u8, email: &'static str, password: &'static str) -> User<'static> {
            User {
                id: Uuid::new_v4(),
                name,
                age,
                email,
                password,
                admin: false
            }
        }
    }

    trait Admin {
        fn turn_admin(&mut self);
    }

    impl Admin for User<'_> {
        fn turn_admin(&mut self) {
            self.admin = true;
        }
    }

    fn find_by_id(id: Uuid, users: Vec<User>) -> User {
        let user = match users.into_iter().find(|user| user.id == id) {
            Some(user) => user,
            None => panic!("No user finded!")
        };

        user
    }

    pub fn user_call() {
        let name = "Eduardo";
        let age = 20;
        let email = "eduardo@gmail.com";
        let password = "123";

        let mut users: Vec<User> = Vec::new();

        users.push(User::new(name, age, email, password));
        users.push(User::new(name, age, email, password));
        users.push(User::new(name, age, email, password));

        println!("{:#?}", users);
        
        let user = &users[0];
        
        let mut user_finded = find_by_id(user.id, users);
        
        user_finded.turn_admin();
        
        println!("{:#?}", user_finded);
    }
}

pub fn user_manage() {
    crate::user_manage::user_manage::user_call();
}