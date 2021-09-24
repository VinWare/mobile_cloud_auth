// extern crate elliptic_curve;
mod elliptic_curve;
mod action;

use elliptic_curve::curve as curve;
use elliptic_curve::point as point;

use curve::EllipticCurve;
use point::Point;
use action::Action;

fn input_action() -> Action {
    println!("Enter 1 for register, 2 for auth, 0 to quit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap_or(-1);
    match n {
        0 => std::process::exit(0),
        1 => Action::Register,
        2 => Action::Authenticate,
        _ => {
            println!("Please enter one of the valid values");
            input_action()
        },
    }
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
    let a = Point {
        x:1,
        y:2
    };
    let b = Point {
        x: 2,
        y: 3
    };
    let c = a.add(&b);
    let e = EllipticCurve {order: 11, o:a, g:b};
    println!("{}",e.order);
    println!("{} {}",c.x,c.y);
    let action = input_action();
    match action {
        Action::Register => register(),
        Action::Authenticate => auth(),
    }
}
