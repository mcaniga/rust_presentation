use std::fs::File;
use std::io::ErrorKind;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// recoverable vs unrecoverable err example
fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // NotFound is recoverable err in our scenario - we can create the file
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// Error handling with anyhow - see `prompt_command` in main.rs

pub fn run_error_handling_demo() {
    open_file();
}
