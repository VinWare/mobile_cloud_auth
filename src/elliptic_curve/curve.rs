// mod self::point;
use super::point as point;
use super::helper as helper;

use point::FinitePoint;
use point::Point;
// use helper::mod_div;
// use helper::mod_exp;
// use helper::mod_mul;
// use helper::mod_m;
use helper::ModField;

#[derive(PartialEq,Eq,Clone,Copy)]
pub struct EllipticCurve {
    pub order: u32,
    // pub o: FinitePoint,
    // pub g: FinitePoint,
    pub p: i32,
    pub a:i32,
    pub b:i32,
}

impl EllipticCurve {
fn sum_finite_points(&self, p: FinitePoint, q: FinitePoint) -> FinitePoint {
    let m = ModField{m:self.p};
    let (num,den) = if p != q {
        let num = m.sub(q.y,p.y);
        let den = m.sub(q.x,p.x);
        (num,den)
    } else {
        let num = m.add(m.mul(3,m.exp(p.x,2)),self.a);
        let den = m.mul(2,p.y);
        (num,den)
    };
    let lambda = m.div(num,den);
    let r_x = m.sub(m.exp(lambda,2),m.add(p.x,q.x));
    let r_y = m.sub(m.mul(lambda,m.sub(p.x,r_x)),p.y);
    FinitePoint{x:r_x,y:r_y}
}
    pub fn add(&self, point_p: Point, point_q: Point) -> Point{
     if let Point::Fin(p) = point_p {
         if let Point::Fin(q) = point_q {
             if p == -q {Point::Inf}
             else {Point::Fin(self.sum_finite_points(p,q))}
         } else {point_p}
     } else {
         point_q
     }
    }
    pub fn sub(&self,p: Point, q: Point) ->Point{
         self.add(p,-q)
    }
    pub fn mul (&self, n: i32, p:Point) -> Point{
        if let Point::Inf = p {
            return Point::Inf;
        }
        let mut p = p;
        let mut n = n;
        let mut res = Point::Inf;
        while n > 0{
            if n & 1 != 0 {res = self.add(res,p);}
            p = self.add(p,p);
            n >>= 1;
        }
        res
    }
}
