// extern crate elliptic_curve;
mod elliptic_curve;
mod action;

use elliptic_curve::curve as curve;
use elliptic_curve::point as point;
// use elliptic_curve::helper as helper;

// use elliptic_curve::helper::ext_euc_algo as ext_euc_algo;
use curve::EllipticCurve;
use point::Point;
use action::Action;

fn input_action() -> Action {
    println!("Enter 1 for register, 2 for auth, 0 to quit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap_or(-1);

            // let x = ext_euc_algo(1759,550);
            // println!("{} {} {}",x.0,x.1,x.2);
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
    let o = Point {
        x:0,
        y:0
    };
    let g = Point {
        x: 1,
        y: 7
    };
    // let c = a.add(&b);
    // let c = -a;
    let e = EllipticCurve {order: 28, o:o, g:g, p: 23, a: 1,b:1};
    println!("{}",e.order);
    let p = Point {
    x:3,
y:10
    };
    let q = Point {
        x:9,
        y:7
    };
    let c = e.add(p,q);
    
    println!("{} {}",c.x,c.y);
    // println!("{} {}",a.x,a.y);
    let action = input_action();
    match action {
        Action::Register => register(),
        Action::Authenticate => auth(),
    }
}
