mod enum_try {
    use std::io;
    
    #[derive(Debug)]
    enum IpAddrKind {
        V4((u8, u8, u8, u8)),
        V6((u128, u128, u128, u128, u128, u128, u128, u128))
    }
    
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }
    
    impl IpAddr {
        fn new(address: String) -> IpAddr {
            match address_type(&address){
                (true, false) => {
                    IpAddr {
                        kind: ip_address_v4_string_to_tuple(&address),
                        address
                    }
                },
                (false, true) => {
                    IpAddr {
                        kind: ip_address_v6_string_to_tuple(&address),
                        address
                    }
                },
                (false, false) => {
                    panic!("Invalid IP address");
                },
                (true, true) => {
                    panic!("Invalid IP address");
                },
            }
        }
    }

    fn address_type(address: &String) -> (bool, bool) {
        let mut is_v4 = true;
        let mut is_v6 = true;

        for c in address.chars() {
            if c == ':' {
                is_v4 = false;
            }

            if c == '.' {
                is_v6 = false;
            }
        }
        
        (is_v4, is_v6)
    }

    fn ip_address_v4_string_to_tuple(address: &String) -> IpAddrKind {
        let address_vec: Vec<u8> = address.split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

        if address_vec.len() != 4 {
            panic!("Invalid IP address");
        }

        let address_tuple: (u8, u8, u8, u8) = (
            address_vec[0],
            address_vec[1],
            address_vec[2],
            address_vec[3]
        );

        IpAddrKind::V4(address_tuple)
    }
    
    fn ip_address_v6_string_to_tuple(address: &String) -> IpAddrKind {
        let address_vec: Vec<u128> = address.split(":")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();

        if address_vec.len() != 8 {
            panic!("Invalid IP address");
        }

        let address_tuple: (u128, u128, u128, u128, u128, u128, u128, u128) = (
            address_vec[0],
            address_vec[1],
            address_vec[2],
            address_vec[3],
            address_vec[4],
            address_vec[5],
            address_vec[6],
            address_vec[7]
        );

        IpAddrKind::V6(address_tuple)
    }
    
    pub fn enum_call() {
        println!("Digite um endereço de IP:");
        
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
