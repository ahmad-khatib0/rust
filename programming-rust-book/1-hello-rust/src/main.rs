fn main() {
    println!("Hello, world!");
}

// the greatest common divisor of tow integers
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; // rust infers the type, let t : u64 = m ;
            m = n;
            n = t;
        }
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(15, 14), 1);
    assert_eq!(gcd(2 * 3 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}