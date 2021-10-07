use std::ops::Neg;
use std::fmt::Display;
use std::fmt;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct FinitePoint {
    pub x: i32,
    pub y: i32,
}
impl Neg for FinitePoint {
    type Output = Self;
    fn neg(self) -> Self::Output {
        FinitePoint{x:self.x, y:-self.y}
    }
}
impl Display for FinitePoint {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"{},{}",self.x,self.y)
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
#[derive(PartialEq, Eq,Copy,Clone)]
pub enum Point {
    Fin(FinitePoint),
    Inf,
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Point::Fin(finite_point) => Point::Fin(-finite_point),
            inf => inf,
        }
    }

}
impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Point::Fin(finite_point) => finite_point.fmt(f),
            Point::Inf => write!(f,"Inf"),
        }
    }
}
