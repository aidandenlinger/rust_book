use rand::Rng;

// entry point
fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess >:(");

        // buffer to hold stdin
        let mut guess = String::new(); // :: is an associated function of the type
                                       // a static method in other languages
                                       // not a method on this specific String obj

        // could also do use std::io; to bring into scope
        std::io::stdin() // from stdin
            .read_line(&mut guess) // read the line into the buffer, returns Result
            .expect("Failed to read line :("); // if Result is Err, panic with msg

        // parse into u32 (needs type to know what type it is!)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // return num, set to guess
            Err(_) => continue, // go back to the top of the loop
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
