use crate::part1::{enums::Coin, structs::Person};

fn compute_sum_of_numbers(numbers: Vec<usize>) -> usize {
    unimplemented!();
}

fn find_even_numbers(numbers: Vec<usize>) -> Vec<usize> {
    unimplemented!();
}

fn find_positive_numbers(numbers: Vec<isize>) -> Vec<isize> {
    unimplemented!();
}

fn find_largest_coin(coins: Vec<Coin>) -> Option<Coin> {
    unimplemented!();
}

fn find_last_number_or_zero(numbers: Vec<usize>) -> usize {
    unimplemented!();
}

fn find_total_age(persons: Vec<Person>) -> usize {
    unimplemented!();
}

fn find_sub_list_from_predicate(
    persons: Vec<Person>,
    predicate: fn(&Person) -> bool,
) -> Vec<Person> {
    unimplemented!();
}

// Duplicates elements in vector
// Example: [1, 5, 2] -> [1, 1, 5, 5, 2, 2]
fn duplicate(elements: Vec<usize>) -> Vec<usize> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sum_of_numbers() {
        assert_eq!(compute_sum_of_numbers(vec![1, 2, 3]), 6);
        assert_eq!(compute_sum_of_numbers(vec![]), 0)
    }

    #[test]
    fn test_find_even_numbers() {
        assert_eq!(find_even_numbers(vec![1, 2, 3, 4]), vec![2, 4])
    }

    #[test]
    fn test_find_positive_numbers() {
        assert_eq!(
            find_positive_numbers(vec![1, -4, -8, 11, -200, -1, 8]),
            vec![1, 11, 8]
        );
        assert_eq!(find_positive_numbers(vec![0]), vec![])
    }

    #[test]
    fn test_find_last_item_or_zero() {
        assert_eq!(find_last_number_or_zero(vec![1, 2, 3]), 3);
        assert_eq!(find_last_number_or_zero(vec![]), 0);
    }

    #[test]
    fn test_find_total_age() {
        assert_eq!(
            find_total_age(vec![
                Person::new("John".to_owned(), "Smith".to_owned(), 25),
                Person::new("Sandra".to_owned(), "White".to_owned(), 19),
                Person::new("Paul".to_owned(), "Wright".to_owned(), 64),
            ]),
            108
        );
    }

    #[test]
    fn test_find_sub_list_from_predicate() {
        let persons = vec![
            Person::new("John".to_owned(), "Smith".to_owned(), 25),
            Person::new("Sandra".to_owned(), "White".to_owned(), 19),
            Person::new("Paul".to_owned(), "Wright".to_owned(), 64),
        ];
        assert_eq!(
            find_sub_list_from_predicate(persons, over_age_30),
            vec![Person::new("Paul".to_owned(), "Wright".to_owned(), 64)]
        );
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(
            duplicate(vec![5, 1, 10, 11, 0]),
            vec![5, 5, 1, 1, 10, 10, 11, 11, 0, 0]
        );
        assert_eq!(duplicate(vec![]), vec![]);
    }

    fn over_age_30(person: &Person) -> bool {
        person.age() > 30
    }
}
