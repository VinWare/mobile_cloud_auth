use std::ops::Neg;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point{x:self.x, y:-self.y}
    }
}
// impl PartialEq for Point {
    // fn eq(&self, other: &Self) -> bool {
        // self.x == other.x && self.y == other.y
    // }
// }
// impl Eq for Point {
// 
// }
