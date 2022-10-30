use crate::part1::enums::Coin;

fn get_value_or_panic(x: Option<i32>) -> i32 {
    unimplemented!()
}

fn get_value_or_zero(x: Option<i32>) -> i32 {
    unimplemented!()
}

fn subtract_one_maybe(x: Option<i32>) -> Option<i32> {
    unimplemented!()
}

/// Divide, if denominator is not 0.
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    unimplemented!()
}

fn convert_if_some(x: Option<Coin>) -> Option<u8> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_or_panic_some() {
        assert_eq!(get_value_or_panic(Some(42)), 42);
    }

    #[test]
    #[should_panic]
    fn test_get_value_or_panic_none() {
        get_value_or_panic(None);
    }

    #[test]
    fn test_get_value_or_zero() {
        assert_eq!(get_value_or_zero(Some(3)), 3);
        assert_eq!(get_value_or_zero(Some(0)), 0);
        assert_eq!(get_value_or_zero(None), 0);
    }

    #[test]
    fn test_subtract_one() {
        assert_eq!(subtract_one_maybe(Some(3)), Some(2));
        assert_eq!(subtract_one_maybe(Some(-1)), Some(-2));
        assert_eq!(subtract_one_maybe(None), None);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(30.1, 2.0), Some(15.05));
        assert_eq!(divide(30.1, 0.0), None);
    }

    #[test]
    fn test_convert_if_some() {
        assert_eq!(convert_if_some(Some(Coin::Nickel)), Some(5));
        assert_eq!(convert_if_some(None), None);
    }
}
