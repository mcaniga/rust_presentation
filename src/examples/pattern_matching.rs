#[derive(Debug)]
enum UsState {
    California,
    Florida,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn print_dice_roll(dice_roll: u32) {
    match dice_roll {
        1 => println!("Dice roll: One"),
        2 => println!("Dice roll: Two"),
        3 => println!("Dice roll: Three"),
        4 => println!("Dice roll: Four"),
        5 => println!("Dice roll: Five"),
        6 => println!("Dice roll: Six"),
        _ => println!("Dice roll: Invalid number"),
    }
}

pub fn run_pattern_matching_demo() {
    println!("nickel: {}", value_in_cents(Coin::Nickel));
    println!("penny: {}", value_in_cents(Coin::Penny));
    println!("dime: {}", value_in_cents(Coin::Dime));
    println!(
        "Florida quarter: {}",
        value_in_cents(Coin::Quarter(UsState::Florida))
    );
    println!(
        "California quarter: {}",
        value_in_cents(Coin::Quarter(UsState::California))
    );
    print_dice_roll(1);
    print_dice_roll(7);
}
