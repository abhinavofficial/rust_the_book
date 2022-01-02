pub mod playing_with_enum {
    //V4 and V6 are variable of IPAddrKindBasic. If to be passed in function, we pass IPAddrKindBasic.
    enum IPAddrKindBasic {
        V4,
        V6,
    }
    struct IPAddrStruct {
        kind: IPAddrKindBasic,
        address: String,
    }
    // Better way to the above.
    // The name of each enum variant that we define, also becomes a function that constructs an instance of the enum.
    // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
    enum IPAddrKind {
        V4(String),
        V6(String),
    }
    //There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // In short, we can define any type in its variant
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    #[derive(Debug)]
    enum USState {
        Alaska,
        Alabama,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(USState)
    }

    impl Coin {
        fn value_in_cents(&self) -> u8 {
            match self {
                Penny => 1,
                Nickel => 5,
                Dime => 10,
                Quarter => 25,
            }
        }
    }

    fn convert_in_dollar_cents() {
        println!("Enter number of Penny")
    }

    fn test() {
        let m = Message::Write(String::from("hello"));
        m.call();

        let cents = Coin::Penny.value_in_cents();
        println!("");

        //if let implementation. Use this when you do not want to handle _. In other cases, match is a better construct.

    }
}