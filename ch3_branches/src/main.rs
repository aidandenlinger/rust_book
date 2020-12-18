fn main() {
    if_basic();
    if_exp();
    loop_basic();
    while_basic();
    for_basic();
}

fn if_basic() {
    let num = 3;

    // don't need parentheses!
    // must be a bool, can't check numbers by themselves (0 isn't false)
    if num == 5 {
        println!("[if_basic] 5!");
    } else if num == 6 {
        println!("[if_basic] 6!");
    } else {
        println!("[if_basic] no one cares about this number");
    }
}

fn if_exp() {
    // if is an expression!
    let cond = true;
    // both results must be the same type so num is typed properly
    let num = if cond { 5 } else { 6 };

    println!("[if_exp] num is {}", num);
}

fn loop_basic() {
    // goes forever
    // loop {
    //     println!("and on");
    // }

    // use break to exit

    // use param to break to get a result from a loop (retry until cond true)
    let mut count = 0;
    let res = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("[loop_basic] res = {}", res);
}

fn while_basic() {
    // naturally there's also a while loop
    let mut num = 3;

    while num != 0 {
        println!("[while_basic] num = {}", num);

        num -= 1;
    }
}

fn for_basic() {
    // and to avoid using while loops to index, we also have a for loop!
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("[for_basic] value is {}", i);
    }

    // get tuples of index and value!
    for (i, v) in a.iter().enumerate() {
        println!("[for_basic] value at {} is {}", i, v)
    }
    // iteration on a range
    for i in 1..4 {
        println!("[for_basic] loop ran {} times", i);
    }

    // or reverse
    for i in (1..4).rev() {
        println!("[for_basic] reverse count: {}", i);
    }
}
