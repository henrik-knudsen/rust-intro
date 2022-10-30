#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    pub fn new(first_name: String, last_name: String, age: u8) -> Person {
        Person {
            first_name,
            last_name,
            age,
        }
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn full_name(&self) -> String {
        format!("{0} {1}", self.first_name(), self.last_name())
    }

    pub fn change_name(&mut self, first_name: String, last_name: String) {
        self.first_name = first_name;
        self.last_name = last_name;
    }
}

fn pretty_print_person(person: &Person) -> String {
    format!(
        "{0} {1} is {2} years old.",
        person.first_name(),
        person.last_name(),
        person.age()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_name() {
        let p = get_person();
        assert_eq!(p.first_name(), "Ola");
    }

    #[test]
    fn test_last_name() {
        let p = get_person();
        assert_eq!(p.last_name(), "Nordmann");
    }

    #[test]
    fn test_age() {
        let p = get_person();
        assert_eq!(p.age(), 30);
    }

    #[test]
    fn test_full_name() {
        let p = get_person();
        assert_eq!(p.full_name(), "Ola Nordmann");
    }

    #[test]
    fn test_change_name() {
        let mut p = get_person();
        assert_eq!(p.full_name(), "Ola Nordmann");

        p.change_name("Per".to_owned(), "Hansen".to_owned());
        assert_eq!(p.full_name(), "Per Hansen");
    }

    #[test]
    fn test_pretty_print_person() {
        let p = get_person();
        assert_eq!(pretty_print_person(&p), "Ola Nordmann is 30 years old.");
    }

    /// Test setup, to create a person
    fn get_person() -> Person {
        Person::new("Ola".to_owned(), "Nordmann".to_owned(), 30)
    }
}
