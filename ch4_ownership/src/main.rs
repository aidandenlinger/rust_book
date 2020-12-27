fn main() {
    var_scope();
    move_clone_copy();
    ownership_and_functions();
    references();
    slices();
}

/*
 * ownership rules are checked at compile time, and thus don't impact
 * runtime performance
 */

/*
 * Stack / heap:
 *
 * Stack is last in first out (push and pop onto stack)
 * Stack data must have a fixed size
 *
 * unknown sized data can go on the heap instead: you request a certain
 * amount of space for the heap and get a pointer to that allocation
 *
 * stack is generally faster
 * - pushing to stack doesn't involve an allocator looking for space
 * - accessing data is faster because heap involves following a ptr
 *
 * when you call a function, parameters and local vars pushed on stack,
 * popped off after
 *
 * heap problems that ownership addresses
 * - what code is using what data
 * - minimize duplicates
 * - cleaning up unused data to not run out of space
 */

/*
 * Ownership rules:
 * - Each value in rust has a variable that's the owner
 * - There can only be one owner at a time
 * - When the owner goes out of scope, the value is dropped
 */

fn var_scope() {
    {
        // v is a string literal with value known at compile time
        let v = "i am v and i exist";
        // v is in the scope of the braces, is valid
        println!("[var_scope] I can do stuff with v: {}", v);
    }
    // v is no longer valid, is out of scope and can't be referenced
    // v

    // Now introducing the String type as an example of heap-allocated data
    // example use case: user input like in ch2, we don't know size
    let mut s = String::from("dynamic string");

    // can be mutated unlike a string literal!
    println!("[var_scope] {}", s);
    s.push_str(" :0");
    println!("[var_scope] {}", s);

    /*
     * why?
     *
     * v is a string literal we know the size of, the text is hardcoded but
     * thus immutable
     *
     * String must allocate an unknown (as of compile time) amount of space on
     * the heap, allows for a mutable growable text
     *
     * String::from requests the memory needed as typical
     * But how can we return the memory to the allocator when we're done with
     * the String?
     *
     * GC approach is to clean up memory that isn't being used anymore
     * other approach is manual management, you're in charge of one allocate
     * and one free
     *
     * Rust approach: return memory once variable owning it goes out of scope
     */

    {
        let _s = String::from("heapbound");
        // _s is valid, we could do stuff
    }
    // the scope is over, _s is no longer bound (doesn't matter that it's heap)

    /*
     * to free memory, rust calls drop for you, defined by author of type
     */
}

fn move_clone_copy() {
    /* Move */

    // the value 5 is binded to x
    let x = 5;
    // we make a copy of the value in x and bind it to y
    let _y = x;

    // this works because integers are simple and have a fixed size! this is
    // on the stack!

    // complex data types
    // a String has three parts: ptr, length, capacity, on the stack
    let s1 = String::from("i am string");
    // we copy the String data to s2, but that means we copy the PTR, not the
    // thing it's pointing to! They're both pointing to the same string buffer!
    let s2 = s1;

    // Ownership rule 2: only one owner at a time. s1 is now invalid and s2 is
    // the owner. nothing is freed

    // won't work
    // println!("[move_clone_copy] {}", s1);

    // will work, since s2 is the owner
    println!("[move_clone_copy] {}", s2);

    // since s1 is invalid, we call this a MOVE. s1 is moved into s2
    // Rust never auto makes deep copies of data. so all copying is inexpensive,
    // but shallow

    /* Clone */

    // so let's say we DO want a deep copy

    let mut s3 = s2.clone(); // visual indicator: this is a clone, it's expensive

    println!("[move_copy_clone] cloned: s2 = {}, s3 = {}", s2, s3);
    s3.push_str(" and i am modified sep from s2");
    println!("[move_copy_clone] cloned: s2 = {}, s3 = {}", s2, s3);

    /* Copy */

    // So why did integers work like clone without an explicit call?

    // Integers completely on stack, deep/shallow copies are just as quick
    // so we just deep copy x into y

    // This is the Copy trait: you can either have the Copy or Drop trait, can't
    // have both.

    // Rule of thumb: simple scalar values can be, if it requires allocation or
    // is a resource it isn't Copy

    // integer types, booleans, floating points, chars, tuples of Copy
}

fn ownership_and_functions() {
    // without return types
    {
        let s = String::from("red shirt");

        takes_ownership(s);

        fn takes_ownership(a_string: String) {
            println!("[takes_ownership] {}", a_string);
        } // a_string is out of scope here: destroyed

        // can't do it, s was moved to takes_ownership
        // println!("{}", s);

        let x = 5;

        copied(x);

        fn copied(y: i32) {
            println!("[copied] y = {}", y);
        }

        println!("[ownership_and_functions] but we can still use x: {}", x);
    }
    // nothing special here: the string from s was destroyed at the end of
    // takes_ownership and was the only heap allocated memory

    // with return values
    {
        let _s1 = give_ownership(); // s1 is owner of string from method

        fn give_ownership() -> String {
            String::from("new string!") // string allocated, moved out
        } // no destruction, because allocated value is returned

        let s2 = String::from("stringy");

        println!("[ownership_and_functions] s2 = {}", s2);

        let s3 = modify_string(s2);

        // doesn't work: s2 was moved
        // println!("[ownership_and_functions] s2 = {}", s2);

        // but this is fine, the function returned into s3
        println!("[ownership_and_functions] s3 = {}", s3);

        fn modify_string(mut old_str: String) -> String {
            old_str.push_str(" string string");
            old_str
        }
    } // now s3 and s1 are destroyed, the only two allocations we made
}

fn references() {
    // What if we want to do an operation on a param without having to hand it
    // back?
    {
        let s1 = String::from("stringy");

        // &s1 refers to s1's data but doesn't own it
        let len = calculate_len(&s1);

        // still able to reference s1, now have len
        println!("[reference] len of {} is {}", s1, len);

        // &str uses a slice rather than an obj, according to clippy.
        // not the most impressive function, i'll admit
        fn calculate_len(input: &str) -> usize {
            // cannot modify a borrowed value by default!
            // (assuming it was &String)
            // input.push_str(" string");
            input.len()
        } // input goes out of scope, but doesn't have ownership, so nothing
          // happens
    }

    // ofc, we can also use mutable references
    {
        let mut s = String::from("i am");

        change(&mut s);

        fn change(input: &mut String) {
            input.push_str("tired");
        }
    }

    // important note tho: you can't have two mutable references at the same
    // time, prevents data races
    {
        let mut s = String::from("one at a time");

        let _r1 = &mut s;
        let _r2 = &mut s;

        // uncommenting this line (which actually uses them) errors r2
        // println!("referencing r1 and r2: {} {}", _r1, _r2);
    }

    // you could create a separate scope for r1 and r2 so no simultaneous
    // mutable references

    // another note: can't have a mutable reference when an immutable one
    // exists: the immutable client assumes the data hasn't changed
    // (reader/writer), can have unlimited immutable refs though

    // note that scope ends the last time a reference is used, so this is ok
    {
        let mut s = String::from("one at a time");
        let r1 = &mut s;

        println!("[references] r1 : {}", r1);

        let r2 = &s;
        // ok: r1 scope is over, the last time it was used was 261
        println!("[references] r2 : {}", r2);

        // NOT ok: extends r1's scope, immutable and mutable at same time
        // println!("[references] r1 : {}, r2 : {}", r1, r2);
    }

    // no dangling references! rust catches when you return a reference to a
    // destroyed object
    {
        // fn dangle() -> &String {
        //     let s = String::from("not long for this world");

        //     &s
        // }

        // doesn't compile, borrowing from a nonexistent object

        // in this case, return the obj directly
    }
}

fn slices() {
    // no ownership
    // reference contiguous sequence of items in a collection instead of the
    // whole thing

    // toy example: get a word from a string. return indexes? what if data
    // changes?

    // better way is to use slices

    // slices hold a start point and a length
    {
        fn first_word(input: &str) -> &str {
            for (i, &item) in input.as_bytes().iter().enumerate() {
                if item == b' ' {
                    return &input[..i]; // from 0 to i, not including i
                }
            }

            &input // if no space, then the whole thing is the first word
        }
        let /*mut*/ s = String::from("the first word of this string is the!");
        let first = first_word(&s);
        // s.clear(); // can't happen, we're borrowing immutably in first
        println!("[slices] first word of '{}' is '{}'", s, first);
    }

    // benefit of this is we return a slice/reference directly, so we're holding
    // onto an immutable reference. any changes to the root data will fail
    // because we're holding onto an immutable reference.

    // note that string literals are slices, a slice pointing to the hardcoded
    // data in the binary. they're immutable references, can't be made mut

    // we can also slice arrays for example
    {
        let a = [1, 2, 3, 4, 5];
        println!("[slices] slice array to first 3 elements: {:?}", &a[..3]);
    }
}
