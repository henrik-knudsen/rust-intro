fn say_hello(name: &str) -> String {
    unimplemented!();
}

fn concat_strings(tuple: (&str, &str)) -> String {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello("John"), "Hello John");
    }

    #[test]
    fn test_concat_strings() {
        assert_eq!(concat_strings(("John", " Smith")), "John Smith");
    }
}
