mod elliptic_curve;
mod action;

use elliptic_curve::curve as curve;
use elliptic_curve::point as point;

use curve::EllipticCurve;
use point::FinitePoint;
use point::Point ;
use action::Action;

struct SystemParams {
    e : EllipticCurve,
    g: FinitePoint ,
    k: FinitePoint,
}
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
fn main() {
    let g = FinitePoint {
        x: 1,
        y: 7
    };
    let e = EllipticCurve {order: 28,  p: 23, a: 1,b:1};
    let s = 3;
    if let Point::Fin(k) = e.mul(s,Point::Fin(g)) {
    let s = SystemParams{
        e:e,
        g:g,
        k:k,
    };
    println!("{}",e.order);
    println!("I'm back");
    // let action = input_action();
    // match action {
    //     Action::Register => register(),
    //     Action::Authenticate => auth(),
    // }
}
}
