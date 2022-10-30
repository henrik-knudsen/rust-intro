fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn max(a: usize, b: usize) -> usize {
    if a >= b {
        a
    } else {
        b
    }
}

fn is_even(n: u128) -> bool {
    n % 2 == 0
}

fn pow(n: usize, exponent: usize) -> usize {
    n.pow(exponent as u32)
}

fn add2(a: u32, b: f64) -> f64 {
    a as f64 + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(4, 3), 7);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(1, 2), 2);
        assert_eq!(max(4, 3), 4);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(4), true);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(3, 3), 27);
        assert_eq!(pow(2, 8), 256);
    }

    #[test]
    fn test_add2() {
        assert_eq!(add2(4, 3.1), 7.1);
        assert_eq!(add2(0, 0.25), 0.25);
    }
}
