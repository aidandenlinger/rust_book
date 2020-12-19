fn main() {
    println!("[fToC] 98.6F = {}C", f_to_c(98.6));
    println!("[cToF] 37C = {}F", c_to_f(37.0));
    println!("[nth_fib_num] 37th fib num: {}", nth_fib_num(37));
    println!();
    twelve_days_of_christmas();
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn nth_fib_num(n: i32) -> i32 {
    if n <= 0 {
        panic!("nth_fib_num requires positive num")
    };

    fib_iterator((0, 1), n)
}

fn fib_iterator(x: (i32, i32), max: i32) -> i32 {
    // there has to be a better way, i want to do x @ (f0, f1) a la haskell
    let (f0, f1) = x;
    if max == 1 {
        f1
    } else {
        fib_iterator((f1, f0 + f1), max - 1)
    }
}

fn twelve_days_of_christmas() {
    for i in 0..12 {
        days_printer(i);
    }
}

const PRESENTS: &[&str] = &[
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const WORDS: &[&str] = &[
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelth",
];

fn days_printer(day: u32) {
    println!(
        "On the {} day of Christmas, my true love sent to me",
        WORDS[day as usize]
    );
    // = makes range inclusive
    for i in (0..=day).rev() {
        println!("{}", PRESENTS[i as usize]);
    }
    println!();
}
