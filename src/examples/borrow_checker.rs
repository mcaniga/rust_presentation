/*
* Rules:
* 1. - Each variable is owned by exactly one owner
* 2. - Variable can be borrowed with immutable reference (safe) or mutable reference (may cause harm)
* 3. - You can create infinite number of immutable references - `&`
* 4. - Only one mutable reference - `&mut` to a variable can be created
* 5. - Don't use & after you have &mut on a variable (`&` can be declared, but not used)
* 6. - Reference must be valid
*/

fn gives_ownership() -> String {
    let s = String::from("it's yours");
    s
}

fn takes_ownership(s: String) {
    println!("{}", s);
} // `drop` is called, `some_string` is freed now

fn takes_and_gives_back(s: String) -> String {
    s
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn run_ownership_demo() {
    let str1 = gives_ownership();
    let str2 = takes_ownership(str1);
    // println!("{}", str1); // `str1` was freed, this will not compile
    let x = 1;
    makes_copy(x);
    println!("x: {}", x); // can use `x` because `i32` has `Copy` trait
}

pub fn run_borrow_checker_rules_demo() {
    let mut s = String::from("hello");
    let r1 = &s; // ok
    let r2 = &s; // ok
    println!("r1: {}, r2: {}", r1, r2); // ok

    let r3 = &mut s; // may not be ok, depends on the usage of r1, r2
                     // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3); } // we are using r1, r2, but mutable reference (r3) is defined, can't compile
    println!("r3: {}", r3); // using r3 only - OK
}

pub fn run_borrow_checker_demo() {
    run_ownership_demo();
    run_borrow_checker_rules_demo();
}
