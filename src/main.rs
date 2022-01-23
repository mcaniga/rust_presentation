mod examples;
use crate::examples::async_example::run_async_demo;
use crate::examples::basics::run_basics_demo;
use crate::examples::borrow_checker::run_borrow_checker_demo;
use crate::examples::collections::run_collections_demo;
use crate::examples::error_handling::run_error_handling_demo;
use crate::examples::generics::run_generics_demo;
use crate::examples::null_handling::run_null_handling_demo;
use crate::examples::pattern_matching::run_pattern_matching_demo;
use crate::examples::rc::run_rc_demo;
use crate::examples::structs::run_structs_demo;
use crate::examples::threads::run_threads_demo;
use crate::examples::unsafe_example::run_unsafe_demo;
use anyhow::anyhow;
use anyhow::Context;
use std::io;

enum Command {
    RunBasics,
    RunStructs,
    RunNullHandling,
    RunPatternMatching,
    RunCollections,
    RunGenerics,
    RunBorrowChecker,
    RunRc,
    RunThreads,
    RunErrorHandling,
    RunUnsafe,
    RunAsync,
    Exit,
}

fn print_menu() {
    println!("What example do you want to run?");
    println!("1. Basics");
    println!("2. Structs");
    println!("3. Null Handling");
    println!("4. Pattern Matching");
    println!("5. Collections");
    println!("6. Generics");
    println!("7. Borrow Checker");
    println!("8. Rc");
    println!("9. Threads");
    println!("10. Error Handling");
    println!("11. Unsafe");
    println!("12. Async");
    println!("13. Exit Program");
    println!("");
}

fn prompt_command() -> anyhow::Result<u32> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .context("Parsing line failed")?;
    let command_num: u32 = guess
        .trim()
        .parse()
        .context("Provide correct number [1-6]")?;
    Ok(command_num)
}

fn parse_command(command_num: u32) -> anyhow::Result<Command> {
    match command_num {
        1 => Ok(Command::RunBasics),
        2 => Ok(Command::RunStructs),
        3 => Ok(Command::RunNullHandling),
        4 => Ok(Command::RunPatternMatching),
        5 => Ok(Command::RunCollections),
        6 => Ok(Command::RunGenerics),
        7 => Ok(Command::RunBorrowChecker),
        8 => Ok(Command::RunRc),
        9 => Ok(Command::RunThreads),
        10 => Ok(Command::RunErrorHandling),
        11 => Ok(Command::RunUnsafe),
        12 => Ok(Command::RunAsync),
        13 => Ok(Command::Exit),
        _ => Err(anyhow!("Wrong command num: {}", command_num)),
    }
}

async fn execute_command(c: Command) {
    match c {
        Command::RunBasics => run_basics_demo(),
        Command::RunStructs => run_structs_demo(),
        Command::RunNullHandling => run_null_handling_demo(),
        Command::RunPatternMatching => run_pattern_matching_demo(),
        Command::RunCollections => run_collections_demo(),
        Command::RunGenerics => run_generics_demo(),
        Command::RunBorrowChecker => run_borrow_checker_demo(),
        Command::RunRc => run_rc_demo(),
        Command::RunThreads => run_threads_demo(),
        Command::RunErrorHandling => run_error_handling_demo(),
        Command::RunUnsafe => run_unsafe_demo(),
        Command::RunAsync => run_async_demo().await,
        _ => panic!("Ilegal state"),
    };
}

fn get_command() -> anyhow::Result<Command> {
    let command_num = prompt_command()?;
    parse_command(command_num)
}

fn get_valid_command() -> Command {
    loop {
        if let Ok(c) = get_command() {
            return c;
        }
    }
}

#[tokio::main]
async fn main() {
    loop {
        print_menu();
        let command = get_valid_command();
        match command {
            Command::Exit => break,
            _ => {
                println!("");
                println!("Command output:");
                execute_command(command).await;
                println!()
            }
        };
    }
}
