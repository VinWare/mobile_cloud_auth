 fn recurse_ext_euc_algo(r_2: i32, r_1:i32, x_2:i32, y_2:i32, x_1:i32,y_1:i32) -> (i32, i32, i32) {
    if r_1 == 0 {return (r_2,x_2,y_2)}
    let q = r_2/r_1;
    let (r_0,x_0,y_0 ) = (r_2-q*r_1, x_2-q*x_1, y_2-q*y_1);
    // println!("{},{},{}",r_0,x_0,y_0);
    recurse_ext_euc_algo(r_1,r_0,x_1,y_1,x_0,y_0)
}
fn ext_euc_algo(first: i32, second: i32) -> (i32,i32,i32) {
    let (r_2,r_1) = (std::cmp::max(first, second), std::cmp::min(first, second));
    recurse_ext_euc_algo(r_2,r_1,1,0,0,1)
}
pub struct ModField {
    pub m: i32
}
impl ModField {
    pub fn add(&self, first: i32, second: i32) ->i32 {
        self.mod_m(self.mod_m(first)+self.mod_m(second))
    }
    pub fn sub(&self, first:i32, second:i32) -> i32 {
         self.add(first,-second)
    }
pub fn mul(&self, first: i32, second: i32) -> i32{
    let mut res = 0;
    // let first = if first < 0 {first + (first/m)*m + m} else {first};
    // let second = if second < 0 {second + (second/m)*m + m} else {second};
    let (mut a, mut b) = (self.mod_m(first), self.mod_m(second));
    while b > 0 {
        if b & 1 != 0 {res = self.add(res,a);}
        a <<= 1;
        b >>= 1;
    }
    return res;
}
pub fn div(&self, num : i32, den: i32) -> i32 {
    // let den = self.mod(den);
            let (_,_,den_inv) = ext_euc_algo(self.m,self.mod_m(den));
            return self.mul(num,den_inv);
}
pub fn exp(&self,base: i32, exp: i32) -> i32 {
    let mut res = 1;
    // let base = 
    let (mut a, mut b) = (self.mod_m(base), self.mod_m(exp));
    while b > 0{
        if b & 1 != 0 {res = self.mul(res,a);}
        a = self.mul(a,a);
        b >>= 1;
    }
    return res;
}
pub fn mod_m(&self, num:i32) -> i32{
    let m = self.m;
    let num = if num < 0 {num + (num/m)*m + m} else {num};
    return num%m;
}
} 
