mod elliptic_curve;

use elliptic_curve::Point;
use elliptic_curve::Action;

fn input_action() -> Action {
    println!("Enter 1 for register, 2 for auth, 0 to quit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
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
    println!("{} {}",c.x,c.y);
    let action = input_action();
    match action {
        Action::Register => register(),
        Action::Authenticate => auth(),
    }
}
