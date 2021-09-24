
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
