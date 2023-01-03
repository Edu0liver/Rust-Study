
mod user_manage {
    use uuid::Uuid;

    #[derive(Clone)]
    #[derive(Debug)]
    struct User {
        id: Uuid,
        name: String,
        age: u8,
        email: String,
        password: String,
        admin: bool
    }

    #[derive(Debug)]
    struct UsersDB {
        users: Vec<User>
    }

    impl User {
        fn new(name: &String, age: u8, email: &String, password: &String) -> User {
            User {
                id: Uuid::new_v4(),
                name: name.to_string(),
                age,
                email: email.to_string(),
                password: password.to_string(),
                admin: false
            }
        }
    }

    impl UsersDB {
        fn new() -> UsersDB {
            UsersDB {
                users: Vec::new()
            }
        }

        fn find_by_id(&mut self, id: Uuid) -> &mut User {
            let user = match self.users.iter_mut().find(|user| user.id == id) {
                Some(user) => user,
                None => panic!("No user finded!")
            };
    
            user
        }
    }

    trait Admin {
        fn turn_admin(&mut self);
    }

    impl Admin for User {
        fn turn_admin(&mut self) {
            self.admin = true;
        }
    }

    pub fn user_call() {
        let name = String::from("Eduardo");
        let age = 20;
        let email = String::from("eduardo@gmail.com");
        let password = String::from("123");

        let mut users_db = UsersDB::new();

        users_db.users.push(User::new(&name, age, &email, &password));
        users_db.users.push(User::new(&name, age, &email, &password));
        users_db.users.push(User::new(&name, age, &email, &password));

        println!("{:#?}", users_db);
        
        let user_finded = users_db.find_by_id(users_db.users[0].id);
        
        user_finded.turn_admin();
        
        println!("{:#?}", user_finded);
    }
}

pub fn user_manage() {
    crate::user_manage::user_manage::user_call();
}