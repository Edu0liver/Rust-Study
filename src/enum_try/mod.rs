use std::str;
use std::str::FromStr;

mod enum_try {
    use std::io;
    
    #[derive(Debug)]
    enum IpAddrKind {
        V4((u8, u8, u8, u8)),
        V6(String)
    }
    
    #[derive(Debug)]
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
        fn some_function() -> String {
            String::from("Olá :)")
        }
    }
    
    impl IpAddr {
        fn new(address: String) -> IpAddr {
            let address_tuple = ip_address_string_to_tuple(&address);

            match address_tuple {
                IpAddrKind::V4(_) => {
                    return IpAddr {
                        kind: address_tuple,
                        address
                    }
                },
                IpAddrKind::V6(_) => {
                    return IpAddr {
                        kind: address_tuple,
                        address
                    }
                }
            }
        }
    }

    fn ip_address_string_to_tuple(address: &String) -> IpAddrKind {
        let address_vec: Vec<u8> = address.split(".").map(|x| {
            match x.parse::<u8>() {
                Ok(n) => n,
                Err(_) => 255
            }
        }).collect();

        if address_vec == [255, 255, 255, 255] {
            return IpAddrKind::V6(address.clone());
        }

        let address_tuple: (u8, u8, u8, u8) = (address_vec[0], address_vec[1], address_vec[2], address_vec[3]);

        IpAddrKind::V4(address_tuple)
    }
    
    pub fn enum_call() {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let ip_address = IpAddr::new(input.trim().to_string());

        println!("{:#?}", ip_address);
    }

}

pub fn enum_try() {
    crate::enum_try::enum_try::enum_call();
}
