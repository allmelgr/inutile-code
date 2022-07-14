pub fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        a + b
    } else {
        gcd(b, a % b )
    }
}