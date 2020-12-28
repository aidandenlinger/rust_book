fn main() {
    enum_basics();
    option();
    match_exp();
}

fn enum_basics() {
    // enums enumerate a number of options
    // enum can only be one option

    {
        enum IpAddrKind {
            V4,
            V6,
        };

        // declaration:
        let _four = IpAddrKind::V4;
        let _six = IpAddrKind::V6;
        // both vars are type IpAddrKind, so we can have funcs that take in
        // IPAddrKind and can take either one
    }

    // enums with data, just add it as a param
    {
        enum IpAddr {
            V4(String), // tuple struct
            V6(String), // tuple struct
        }

        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }

    // fancier enum declarations
    {
        enum _Message {
            Quit,                       // unit struct
            Move { x: i32, y: i32 },    // struct
            Write(String),              // tuple struct
            ChangeColor(i32, i32, i32), // tuple struct
        }

        // we can define methods on enums!

        impl _Message {
            fn _offline() -> String {
                String::from("i am offline >:(")
            }
        }
    }
}

fn option() {
    // Option: an enum for when a result is something or nothing, Rust's
    // representation of null

    // In the beginning null was created. This had made many people very angry
    // and has been widely regarded as a bad move

    // Option is Option<T> {Some(T), None}
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // not okay: we can't add an Option to an i8
    // let sum = _x + _y;

    // how do we operate on options? One way is the match expression...
}

fn match_exp() {
    {
        #[derive(Debug)]
        enum UsState {
            // today, we only care about states that start with C
            California,
            _Chicago,
            _Conneticut,
        }

        enum Coin {
            Penny,
            _Nickel,
            _Dime,
            Quarter(UsState), // the only one that takes a param
        }

        // this just made more sense to me as a method rather than a function
        impl Coin {
            fn to_cents(&self) -> u8 {
                // very haskell :) the condition for match can be any type, then we
                // list patterns and the arm => for code to run
                // matches patterns in order, resulting value of the exp is the
                // value returned
                // use curly brackets to run more lines
                match self {
                    Coin::Penny => 1,
                    Coin::_Nickel => 5,
                    Coin::_Dime => 10,
                    Coin::Quarter(s) => {
                        // s is the state of the quarter, matched in the pattern
                        println!("[to_cents] This quarter is from {:?}", s);
                        25
                    }
                }
            }
        }

        println!(
            "[match_exp] the value of a penny is {}",
            Coin::Penny.to_cents()
        );
        println!(
            "[match_exp] the value of a quarter is {}",
            Coin::Quarter(UsState::California).to_cents()
        );
    }

    // Option<T>
    {
        fn inc(x: Option<i32>) -> Option<i32> {
            match x {
                Some(n) => Some(n + 1), // do an inc
                None => None,           // carry on null
            }
        }

        assert_eq!(Some(6), inc(Some(5)));
        assert_eq!(None, inc(None));
    }

    // exhaustive patterns/ using _
    {
        let a_u8_num = 0u8;
        match a_u8_num {
            // we have to match EVERY u8 value here or won't compile
            1 => println!("One"),
            _ => println!("not one, who cares"), // this matches all patterns we haven't matched :)
        }
    }

    // combine if and let to handle matching on one value!
    {
        // reimplementing the example from above, but in an option to show
        // that this is pattern matching (we can match params and stuff, just
        // too lazy to recopy the Coin def)
        let a_u8_num = Some(1u8);
        if let Some(1) = a_u8_num {
            println!("One!");
        } else {
            println!("still, no one cares about non-ones");
        }
    }
}
