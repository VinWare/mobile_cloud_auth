// mod self::point;
use super::point as point;

use point::Point;

pub struct EllipticCurve {
    pub order: u32,
    pub o: Point,
    pub g: Point,
}

impl EllipticCurve {
    pub fn add(&self, a: Point, b: Point) -> Point {
        if a == self.o { return b }
        else if b == self.o {return a} 
        return Point{x:0,y:0}
        // if (a == o)
    }
}
