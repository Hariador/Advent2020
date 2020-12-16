
fn main() {
    let deg:f64 = -90.0;
    let x = 100;
    let y = -56;
    let x1 = x as f64;
    let y1 = y as f64;
    println!("{}", y as f64);
    let xt = x1 * deg.to_radians().cos() - y1 * deg.to_radians().sin();
    let yt = x1 * deg.to_radians().sin() + y1 * deg.to_radians().cos();
    println!("DEG: {:?}", deg.to_radians());
    println!("X: {} Y:{}", x1,y1);
    println!("Xt: {} yt: {}", xt.round() as i32, yt as i32);
    let xt = x1 * deg.to_radians().cos() - y1 * deg.to_radians().sin();
    let yt = x1 * deg.to_radians().sin() + y1 * deg.to_radians().cos();
    let x1 = xt;
    let y1 = yt;
    let deg:f64 = 90.0;
    println!("DEG: {:?}", deg.to_radians());
    println!("X: {} Y:{}", x1,y1);
    println!("Xt: {} yt: {}", xt.round() as i32, yt as i32);
}