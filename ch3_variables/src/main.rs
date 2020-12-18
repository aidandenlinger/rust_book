fn main() {
    // Immutability

    // let x = 5; // causes error with line 5
    let mut x = 5; // needs to explicitly be immutable
    println!("[Immutability] The value of x is: {}", x);
    x = 6; // if no mut, this would cause compiler error
    println!("[Immutability] The value of x is: {}", x);

    println!();

    // Constants

    /*
        can't use mut with consts: const is stronger, will always be immutable
        any scope
        can only be set to a constant, cannot be set via function call or other
        runtime dependent value

        underscores can be put in numeral literals! 100000 = 100_000

        must have a type annotation
    */
    const BAD_PI: f64 = 3.141_58; // end in 8 to shut clippy up
    println!("[Constants] Bad approximation of Pi: {}", BAD_PI);
    // IMPORTANT_NUMBER = 3; // naturally doesn't work with const

    println!();

    // Shadowing

    /*
        Can reuse the variable name "x" by using a let
        different than mut, because we have to use let to shadow. the variable
        is immutable after the let, while mut would always be mutable.
    */

    let y = 3;
    println!("[Shadowing] y = {}", y);
    let y = y + 1;
    println!("[Shadowing] y = {}", y);
    let y = y + 2;
    println!("[Shadowing] y = {}", y);

    // since we're creating a new variable, just taking over the name, it can
    // be a different type. can prevent stuff like "y_num" and "y_len"

    let y = "I'm a string now";
    println!("[Shadowing] y = {}", y);

    // doesn't work with mut, mut fixes type
    let mut _z = 5;
    // _z = "i'm also a string now!"; // error: expected integer

    println!();

    // Scalar types

    // default to i32
    // can use type suffixes, and as said before, _ as a visual separator
    let num = 5u8;

    // from https://stackoverflow.com/questions/21747136/
    // takes in reference to parameter (we don't care about the actual value)
    // to get the type, hand that type to type_name
    fn print_type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    println!("[Scalar Types] num is {}", print_type_of(&num));
    println!();

    // overflow is checked in debug compilations but not release!

    // floating (f32 f64), math, booleans, chars work as expected

    // tuples have fixed length. each position has a type, but can be different
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // per https://doc.rust-lang.org/std/fmt/, :? forces the Debug output to
    // print tuple since tuple doesn't implement Display
    println!("[Compound Types] {:?}", tup);
    // can pattern match a tuple!
    let (x, y, z) = tup;
    println!("[Compound Types] ({}, {}, {})", x, y, z);

    // can also access tuple types via periods
    println!("[Compound Types] ({}, {}, {})", tup.0, tup.1, tup.2);
    println!();

    // arrays have a fixed length, ensure data is on stack rather than heap
    // vector is oftentimes more useful, can change size

    // types are [type; num]
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // same value for each element? set equal to [value; num]
    let a = [3; 5];
    println!("[Arrays] [3;5] = {:?}", a);

    // access as normal. what happens with invalid access?
    // let _element = a[10];
    // compiles fine, but runtime errors before the memory is accessed
}
