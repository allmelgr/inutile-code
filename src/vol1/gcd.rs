/*
 *
 */

 fn gcd(a: i32, b: i32) -> i32 {
     if a == 0 {
         b
     }
     else if a < b {
         gcd(b % a, a)
     }
     else {
         gcd(b, a)
     }
 }
 fn main() {
     println!("{}", gcd(10, 9))
 }