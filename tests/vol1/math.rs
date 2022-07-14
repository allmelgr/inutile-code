


#[cfg(test)]
mod math_test {
    use crate::common::*;
    use inutile_code::vol1::math::gcd;
    #[test]
    fn test_gcd1() {
        setup();
        assert_eq!(gcd(1, 100), 1);
        assert_eq!(gcd(2,1024), 2);
        assert_eq!(gcd(1024,2), 2);
        assert_eq!(gcd(1024,1023), 1);
        assert_eq!(gcd(0,1000), 1000);
        destroy();
    }
}