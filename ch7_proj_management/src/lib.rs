// Packages and crates:
// Crate is a binary or libary
// crate root is a source file that makes the root module of the crate
// package is 1+ crates for functionality with a Cargo.toml file
// package cannot hold more than 1 library crate (unlimited binary crates)

// what does cargo new do?
// Creates a new package (because it has a Cargo.toml file)
// src/main.rs is the crate root of a binary crate with same name as package
// src/lib.rs is a library crate with same name as package, lib.rs is root crate

// so, cargo new creates a binary crate with same name as package
// if a package has main.rs and lib.rs, has two packages: library and binary,
// same name as package. We can have multiple binary crates by using src/bin,
// with each fuile being a separate binary crate

// crate's functonality is namespaced in its scope

// Onto modules!

// modules let us organize code within a crate
// also controls privacy, public for outside code and private for internal use

// modules can hold modules, definitions for anything (structs, enums, funcs,
// etc)

// module tree:
// crate (crate roots form the module named crate)
//  └── front_of_house (child of crate, parent of hosting, etc)
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// by default, modules are private
// parents can't use private children, but children can use everything in
// parents, pub makes something public
mod front_of_house {
    pub mod hosting {
        // pub hosting doesn't make contents pub! Only allows access to hosting
        pub fn add_to_waitlist() {}
        #[allow(dead_code)]
        fn seat_at_table() {}
    }

    #[allow(dead_code)]
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// to use things within a module tree, we can use an absolute or relative path
pub fn eat_at_restaurant() {
    // absolute, crate is root
    // front_of_house is same level as eat_at_restaurant, so no pub
    crate::front_of_house::hosting::add_to_waitlist();

    // relative, starting at the same level as this function (eat_at_restraunt)
    front_of_house::hosting::add_to_waitlist();
}

// accessing parent information:

#[allow(dead_code)]
fn serve_order() {}

#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // prefix with super
    }

    fn cook_order() {}
}

// pub for structs and enums

// a pub struct still has private fields by default
#[allow(dead_code)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String, // private field
}

impl Breakfast {
    #[allow(dead_code)]
    pub fn create() -> Breakfast {
        // Since it's a pub struct with private fields, we can't create the
        // struct without some public function that will return one
        unimplemented!()
    }
}

// ex client can choose type of toast, but chef chooses seasonal_fruit

// enums are different! pub enum makes all variants public!

pub enum Appetizer {
    Soup,  // public
    Salad, // public
}

// use: like a symbolic link. now we can do hosting::add_to_waitlist()
// use crate::front_of_house::hosting; // absolute
// use self::front_of_house::hosting; // relative

// specify a name
#[allow(unused_imports)]
use std::io::Result as IoResult;
// now IoResult refers to std::io::Result

// pub use
pub use crate::front_of_house::hosting;

// so now it's like hosting was defined in this file as pub, external code can
// use add_to_waitlist

// need to import from std library, ex use std::collections::HashMap;

// nested paths
// use std::cmp::Ordering;
// use std::collections::HashMap;
// same as
#[allow(unused_imports)]
use std::{cmp::Ordering, collections::HashMap};

//can use self
// use std::io;
// use std::io::Write;
// same as
#[allow(unused_imports)]
use std::io::{self, Write};

//glob
#[allow(unused_imports)]
use std::collections::*;
// brings all public items in collections into scope
