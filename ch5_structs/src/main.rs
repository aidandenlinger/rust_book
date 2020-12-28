fn main() {
    declaring_structs();
    tuple_structs();
    rectangles();
}

// structs are like tuples in the sense that they hold different data types
// you name each field to easily access later

fn declaring_structs() {
    struct User {
        username: String,
        email: String,
        _sign_in_count: u64,
        _active: bool,
    }

    let user1 = User {
        email: String::from("generic@email.com"),
        username: String::from("xXx_genericUsername_xXx"),
        _active: true, // can declare fields in any order
        _sign_in_count: 1,
    };

    // access with dot notation
    println!(
        "[declaring_structs] user1\tname:{}\temail:{}",
        user1.username, user1.email
    );

    // naturally, by default user1 is immutable. We can declare an entire struct
    // to be mutable, but not individual fields

    // let mut user2 = User {...}

    // if we have vars with the same names as a struct field, we can use field
    // init shorthand
    fn _build_user(email: String, username: String) -> User {
        User {
            email, // field init shorthand, fills with value for email
            username,
            _active: true,
            _sign_in_count: 1,
        }
    }

    // we can also reuse fields of a struct, ex user2 has a different email than
    // user1 but the same everything else
    let user2 = User {
        email: String::from("unique@email.com"),
        ..user1 // fill the rest from user1
    };

    // note that I can't check against user1 anymore: the ..user1 made user2 the
    // owner of the Strings. Need to learn lifetimes to use references, or need
    // to clone the strings and not use the struct update syntax
    println!(
        "[declaring_structs] user2\tname:{}\temail:{}",
        user2.username, user2.email
    );
}

fn tuple_structs() {
    // structs with no names to fields, just the types
    // useful to give a tuple a name and have it be a different type

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // black and origin are different types, a func taking a color couldn't take
    // a point despite them both being tuples of i32s

    // otherwise just like tuples, black.0 will access the first elem of black
}

// unit-like struct, it do jus be empty tho, useful for putting traits on a type
struct _NoThoughtsHeadEmpty {}

fn rectangles() {
    #[derive(Debug)] // automatically derive the Debug trait to be printed
    struct Rectangle {
        height: u32, // named for clarity: otherwise which is height and width?
        width: u32,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // no method syntax approach
    {
        fn area(rect: &Rectangle) -> u32 {
            rect.height * rect.width
        }

        println!(
            // {:?} uses Debug, # new lines the value of the structs
            "[rectangles no method] rect1 is {:#?} and its area is {}",
            rect1,
            area(&rect1)
        );
    }

    // method syntax approach
    {
        // methods are defined within the context of a struct, enum or trait obj
        // first paramter is always &self
        impl Rectangle {
            // implementation block, says what we can do with this struct
            fn area(&self) -> u32 {
                // &self is shorthand for self : &Rectangle in impl block
                // &mut self is fine is we want to edit
                // self would also be fine, but would prevent the initial instance
                // from being used again since we take ownership
                self.width * self.height
            }
        }

        println!(
            // {:?} uses Debug, # new lines the value of the structs
            "[rectangles method] rect1 is {:#?} and its area is {}",
            rect1,
            rect1.area() // automatic referencing and dereferencing, same as
                         // (&rect1).area()
        );

        // realistically this shouldn't be in a separate impl block, it's
        // allowed but bad form. This makes it chronological with the book
        // and demonstrates that multiple impl blocks are allowed though, so
        // we'll take it
        impl Rectangle {
            // other params: just list them after self
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let small_rect = Rectangle {
            width: 1,
            height: 1,
        };

        println!(
            "[rectangles method] rect1 can hold rect2: {}",
            rect1.can_hold(&small_rect)
        );

        // associated functions! (not methods)
        // functions that don't take self, not methods! Use ::, like
        // String::from. unique from any instance
        impl Rectangle {
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        println!(
            "[rectangles method] made a square: {:#?}",
            Rectangle::square(4)
        );
    }
}
