// mod self::point;
use super::point as point;
use super::helper as helper;

use point::Point;
use helper::mod_div;
use helper::mod_exp;
use helper::mod_mul;
use helper::mod_m;
pub struct EllipticCurve {
    pub order: u32,
    pub o: Point,
    pub g: Point,
    pub p: i32,
    pub a:i32,
    pub b:i32,
}

impl EllipticCurve {
    pub fn add(&self, p: Point, q: Point) -> Point {
        if p == self.o { return q }
        else if q == self.o {return p} 
        if p == -q {return self.o}
        let (num,den) = if p != q {
            // let x = ext_euc_algo(1759,550);
            // println!("{} {} {}",x.0,x.1,x.2);
            let num = mod_m(q.y-p.y,self.p);
            let den = mod_m(q.x-p.x,self.p);
            // if(self.m > den)
            // mod_div(num,den,self.p)
            (num,den)
        } else {
            let num = (mod_mul(3,mod_exp(2,p.x,self.p),self.p) + self.a) % self.p;
            let den = mod_mul(2,p.y,self.p);
            // mod_div(num,den,self.p)
            (num,den )
            // ext_euc_algo(1759,550);
        };
        
        let lambda = mod_div(num,den,self.p);
        println!("{} {} {}",num,den,lambda);
    let r_x = mod_m(mod_exp(lambda,2,self.p) - p.x - q.x , self.p);
        println!("{}",mod_exp(lambda,2,self.p));
        let r_y = mod_m(mod_mul(lambda,p.x-r_x,self.p) - p.y , self.p);
        Point{x:r_x,y:r_y}
        // if (a == o)
    }
    pub fn sub(&self,p: Point, q: Point) ->Point{
         self.add(p,-q)
    }
    pub fn mul (&self, n: i32, p:Point) -> Point {
        let mut p = p;
        let mut n = n;
        let mut res = self.o;
        while n > 0{
            if n & 1 != 0 {res = self.add(res,p);}
            p = self.add(p,p);
            n >>= 1;
        }
        res
    }
}
