/// C-style enum, with a set of values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    pub fn value_in_cents(&self) -> u8 {
        unimplemented!()
    }
}

fn max(coin1: Coin, coin2: Coin) -> Coin {
    unimplemented!()
}

/// Complex enum (commonly known as Algebraic datatypes), which can have different kinds of values.
///
// NB: These values all take up different amount of space. Enums are still stored on the stack by default.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or regular struct
    Click { x: i64, y: i64 },
}

impl WebEvent {
    /// Returns true if event was some sort of user interaction
    pub fn is_user_interaction(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_in_cents() {
        assert_eq!((Coin::Penny).value_in_cents(), 1);
        assert_eq!((Coin::Nickel).value_in_cents(), 5);
        assert_eq!((Coin::Dime).value_in_cents(), 10);
        assert_eq!((Coin::Quarter).value_in_cents(), 25);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(Coin::Penny, Coin::Nickel), Coin::Nickel);
        assert_eq!(max(Coin::Penny, Coin::Dime), Coin::Dime);
        assert_eq!(max(Coin::Penny, Coin::Quarter), Coin::Quarter);

        assert_eq!(max(Coin::Nickel, Coin::Penny), Coin::Nickel);
        assert_eq!(max(Coin::Nickel, Coin::Dime), Coin::Dime);
        assert_eq!(max(Coin::Nickel, Coin::Quarter), Coin::Quarter);

        assert_eq!(max(Coin::Dime, Coin::Penny), Coin::Dime);
        assert_eq!(max(Coin::Dime, Coin::Nickel), Coin::Dime);
        assert_eq!(max(Coin::Dime, Coin::Quarter), Coin::Quarter);

        assert_eq!(max(Coin::Quarter, Coin::Penny), Coin::Quarter);
        assert_eq!(max(Coin::Quarter, Coin::Nickel), Coin::Quarter);
        assert_eq!(max(Coin::Quarter, Coin::Dime), Coin::Quarter);
    }

    #[test]
    fn test_is_user_interaction() {
        assert!(!(WebEvent::PageLoad).is_user_interaction());
        assert!(!(WebEvent::PageLoad).is_user_interaction());
        assert!((WebEvent::KeyPress('c')).is_user_interaction());
        assert!((WebEvent::Paste("Hello world".to_owned())).is_user_interaction());
        assert!((WebEvent::Click { x: 0, y: 0 }).is_user_interaction());
    }
}
