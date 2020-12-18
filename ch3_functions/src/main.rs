fn main() {
    println!("[Main] Hello, world!");

    another_function();
    param(5, 10);
}

fn another_function() {
    // functions can be defined in any order, not like C
    println!("[another_function] wowza, another_function");
}

// must declare parameter type
fn param(x: i32, y: i32) {
    println!("[param] Received params {} {}", x, y);
}

// statements perform some action and don't return a value
// expressions evaluate to an expression

// function declarations are statements, calling the function is an expression
fn _statements() {
    // this is a statement.
    // (however, the 6 itself is an expression that returns 6)
    let _y = 6;

    // this isn't ok, because let y = 6 is a statement, nothing is returned to x
    // let x = (let y = 6);
}

fn _expressions() {
    let _y = {
        // This block is an expression returning x+1
        let x = 3;
        // this is an expression. expressions don't have ;
        // ; endings turn it into an expression which doesn't return anything
        x + 1
    };
}

fn _plus_one(x: i32) -> i32 {
    // ok, is an expression that returns
    x + 1
    // not ok: the semicolon turns it to a statement, returns nothing
    // x+1;
}
