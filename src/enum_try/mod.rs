
mod enum_try {
    
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String)
    }
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }

    enum Message {
        Quit,
        Move{ x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    impl Message {
        fn some_function() {
            println!("Olá");
        }
    }
    
    pub fn enum_call() {
        let message = Message::some_function();
        println!("{:#?}", message);
    }

}

pub fn enum_try() {
    crate::enum_try::enum_try::enum_call();
}
