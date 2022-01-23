pub fn print_stuff_from_memory() {
    let mut num = 5;

    // defining pointer to arbitrary memory is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // using that pointer is NOT safe
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub fn run_unsafe_demo() {
    print_stuff_from_memory();
}
