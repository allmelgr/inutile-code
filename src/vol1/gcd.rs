pub fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        b
    }
    else if a < b {
        gcd(b % a, a)
    }
    else {
        gcd(b, a)
    }
}