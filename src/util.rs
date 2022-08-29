/// Returns the factorial of the number
///
/// Defined as
/// ```plain
/// n! = n * (n - 1)!
/// 1! = 0! = 1
/// ```
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        10 => 3628800,
        11 => 39916800,
        12 => 479001600,
        13 => 6227020800,
        14 => 87178291200,
        15 => 1307674368000,
        16 => 20922789888000,
        17 => 355687428096000,
        18 => 6402373705728000,
        19 => 121645100408832000,
        20 => 2432902008176640000,
        _ => n * factorial(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_works() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5_040);
        assert_eq!(factorial(8), 40_320);
        assert_eq!(factorial(9), 362_880);
    }
}
