struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub trait Greet {
    fn greet(&self);
}

impl User {
    fn print_sign_in_count(&self) {
        println!("Sign in count: {}", self.sign_in_count);
    }
    fn static_method() {
        // instance method without `&self` is static
        println!("Dumb static method example");
    }
}

impl Greet for User {
    fn greet(&self) {
        println!("Hello, my name is {}", self.username);
    }
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}

fn login(user: &User) -> User {
    User {
        // rewrite to ..user on and try on linux
        email: user.email.clone(),
        username: user.username.clone(),
        active: user.active,
        sign_in_count: 1,
    }
}

pub fn run_structs_demo() {
    let new_user = build_user("bob123@gmail.com", "bob123");
    let updated_user = login(&new_user);
    new_user.print_sign_in_count();
    new_user.print_sign_in_count();
    updated_user.greet();
    User::static_method();
}
