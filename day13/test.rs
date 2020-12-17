use std::env;

fn main() {
    let a = env::args().nth(1).expect("INT").parse::<i64>().expect("INT");
    let b = env::args().nth(2).expect("INT").parse::<i64>().expect("INT");
    println!("{:?}", mod_inv(a, b));
    
}

fn egcd(y: i64, base: i64 ) -> (i64, i64, i64) {
    if y == 0 {
        (base, 0, 1)
    } else {
        let (gcd, bc_a, bc_b) = egcd(base % y, y);
        (gcd, bc_b - ( base / y) * bc_a, bc_a)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (gcd, x, _) = egcd(x,n);
    return (x % n + n) % n
}