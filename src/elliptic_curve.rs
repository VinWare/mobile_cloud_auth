pub struct Point {
    pub x: i32,
    pub y: i32,
}
struct EllipticCurve {
    order: u32,
    O: Point,
    G: Point,
}
impl Point {
    pub fn add(&self, other: &Point) -> Point {
        return Point {
            x:self.x+other.x,
            y:self.y+other.y
        }
    }
}
