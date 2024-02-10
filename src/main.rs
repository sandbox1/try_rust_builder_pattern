mod builder1;
mod builder2;
mod user;

use builder1::UserBuilder1;
use builder2::UserBuilder2;
use user::User;

fn print_user(method: &str, user: User) {
    println!("{}", method);
    println!("\tUsername: {}", user.username);
    println!("\tEmail: {}", user.email);
    println!("\tSign In Count: {}", user.sign_in_count);
    println!("\tActive: {}", user.active);
    println!()
}

// 1. Struct Literal
fn create_by_struct_literal() -> User {
    let user1 = User {
        email: String::from("hello@hello.xyz"),
        username: String::from("hello"),
        active: true,
        sign_in_count: 1,
    };

    user1
}

// 2. Create using Associated Function
// Read more: https://rust-unofficial.github.io/patterns/idioms/ctor.html
fn create_by_associated_function() -> User {
    User::new(String::from("hello@hello.xyz"), String::from("hello"))
}

// 3. Create using Default trait
// Could also be #[derive(Default)] when all fields implement default
fn create_by_default_trait() -> User {
    User::default()
}

// 4. Create User using Builder Pattern with methods taking implicit reference to self
fn create_by_builder_pattern() -> User {
    let mut builder = UserBuilder1::new();
    builder.username(String::from("username"));
    builder.username(String::from("hello"));
    builder
        .email(String::from("hello@builder.xyz"))
        .sign_in_count(4)
        .active(true);

    println!("Call build to Print::  {:?}", builder.build());
    builder.build()
}

// 5. Create using Builder Pattern with consuming self
fn create_by_builder_pattern2() -> User {
    let builder = UserBuilder2::new()
        .username(String::from("hello"))
        .email(String::from("xxxxx@xxxxxxxx"))
        .sign_in_count(1)
        .active(true);

    // Can't call build twice as build now consumes self
    //println!("{:?}", builder.build());
    builder.build()
}

fn main() {
    print_user("1. struct literal", create_by_struct_literal());
    print_user("2. associated function", create_by_associated_function());
    print_user("3. default trait", create_by_default_trait());
    print_user("4. builder pattern 1", create_by_builder_pattern());
    print_user("5. builder patter 2", create_by_builder_pattern2());
}
