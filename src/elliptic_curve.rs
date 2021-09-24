// mod point;

// use point::Point;

pub enum Action {
    Register,
    Authenticate,
}
struct EllipticCurve {
    order: u32,
    o: Point,
    g: Point,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        return Point {
            x:self.x+other.x,
            y:self.y+other.y
        }
    }
}
