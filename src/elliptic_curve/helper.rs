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
pub fn mod_mul(first: i32, second: i32, m: i32) -> i32{
    let mut res = 0;
    // let first = if first < 0 {first + (first/m)*m + m} else {first};
    // let second = if second < 0 {second + (second/m)*m + m} else {second};
    let (mut a, mut b) = (mod_m(first, m), mod_m(second,m));
    while b > 0 {
        if b & 1 != 0 {res = (res + a) % m;}
        a <<= 1;
        b >>= 1;
    }
    return res;
}
pub fn mod_div(num : i32, den: i32, m: i32) -> i32 {
    let den = mod_m(den,m);
            let (_,_,den_inv) = ext_euc_algo(m,den%m);
            return mod_mul(num,den_inv,m);
}
pub fn mod_exp(base: i32, exp: i32, m: i32) -> i32 {
    let mut res = 1;
    // let base = 
    let (mut a, mut b) = (mod_m(base,m), mod_m(exp,m));
    while b > 0{
        if b & 1 != 0 {res = mod_mul(res,a,m);}
        a = mod_mul(a,a,m);
        b >>= 1;
    }
    return res;
}
pub fn mod_m(num:i32,m:i32) -> i32{
    let num = if num < 0 {num + (num/m)*m + m} else {num};
    return num%m;
}
