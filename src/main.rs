// use std::env;
mod setup;
mod other;
fn input_action() -> i32{
    println!("Enter 1 for register, 2 for auth");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    return n
}
fn register() {
    println!("register")
}
fn auth() {
    println!("auth")
}
// fn read_env() {
    // match env::var("ECC") {
        // Ok(val) => println!("{}: {:?}", "ECC", val),
        // Err(e) => println!("{}",e),
    // };
    // println!("{}",ecc);
// }
fn main() {
    // read_env();
    setup::setup();
    other::other();
    let action = input_action();
    // println!("{}",action);
    match action {
        1 => register(),
        2 => auth(),
        _ => {println!("error");std::process::exit(1)}
    }
    // println!("Hello, world!");
}
