use std::fmt::Debug;

// enum Option<T> {
//     None,
//     Some(T),
// }

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
}

fn get_product(id: u32) -> Option<Product> {
    println!("Getting from DB...");
    match id {
        1 => Some(Product {
            id: 1,
            name: String::from("IPhone X"),
        }),
        2 => Some(Product {
            id: 2,
            name: String::from("Xiaomi Redmi Note 7"),
        }),
        _ => None,
    }
}

pub fn run_null_handling_demo() {
    println!("{:?}", get_product(1));
    println!("{:?}", get_product(3));
}
