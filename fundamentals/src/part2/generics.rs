/// Container type for some generic type T.
pub struct Container<T> {
    value: T,
}

/// Enum for some generic type T. Can hold 0, 1 or 2 values of type T.
pub enum Value<T> {
    Double(T, T),
    Single(T),
    None,
}

/// Trait for some generic type T.
pub trait SomeTrait<T> {
    fn method() -> T;
}

/// Function taking a parameter with generic type T, and returning a value with another generic type U.
pub fn func<T, U>(parameter: T) -> U {
    todo!()
}
