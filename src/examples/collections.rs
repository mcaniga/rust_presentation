pub fn run_vector_demo() {
    let mut v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // let fourth: &i32 = &v[3]; // direct access with index out of bounds - causes panic

    // safe access with index out of bounds - does compile
    match v.get(4) {
        Some(third) => println!("The fourth element is {}", third),
        None => println!("There is no fourth element."),
    }

    // pushing element at the end of the vector
    v.push(4);

    // iterating with immutable reference
    for i in &v {
        println!("{}", i);
    }
    println!();
    v.iter().for_each(|i| println!("{}", i));

    // iterating with mutable reference
    for i in &mut v {
        *i += 50;
    }
    let v2: Vec<i32> = v.iter().map(|i| i * 2).collect();
    v2.iter().for_each(|i| println!("{}", i));
}

use std::collections::HashMap;
pub fn run_hashmap_demo() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // insert if entry does not exist
    scores.entry(String::from("Blue")).or_insert(50);

    // owerwrite entry
    scores.insert(String::from("Blue"), 25);

    // iterate a hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.iter().for_each(|(k, v)| println!("{}: {}", k, v));

    // getting a value from hashmap
    match scores.get("Orange") {
        Some(val) => println!("Score of orange team is: {}", val),
        None => println!("Orange team was not present"),
    }
}

pub fn run_collections_demo() {
    run_vector_demo();
    run_hashmap_demo();
}
