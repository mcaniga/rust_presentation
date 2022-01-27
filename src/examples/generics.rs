use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

fn print_clones<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}", t.clone());
    println!("{:?}", u.clone())
}

pub fn run_generics_demo() {
    let num = 32;
    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };
    print_clones(&num, &tweet);
}
