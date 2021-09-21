// use std::env;
mod other;
mod setup;
mod elliptic_curve;

use elliptic_curve::Point;

fn input_action() -> i32 {
    println!("Enter 1 for register, 2 for auth");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    return n;
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
    let a = Point {
        x:1,
        y:2
    };
    let b = Point {
        x: 2,
        y: 3
    };
    let c = a.add(&b);
    println!("{} {}",c.x,c.y);
    let action = input_action();
    match action {
        1 => register(),
        2 => auth(),
        _ => {
            println!("error");
            std::process::exit(1)
        }
    }
    // println!("Hello, world!");
}
