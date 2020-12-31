mod front_of_house; // ; means load the contents of the module from another file
                    // with the same name (in this case, front_of_house.rs)

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
