mod vol1;
use vol1::math::gcd;

fn main() {
    println!("Hello, world!");
    println!("{}", gcd(10, 9));
    println!("{}", gcd(0, 9));
    println!("{}", gcd(9, 12));
    println!("{}", gcd(12, 9));
}
