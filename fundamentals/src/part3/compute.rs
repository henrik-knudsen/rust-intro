#![allow(unused_mut)]
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::part1::structs::Person;

const NUM_THREADS: usize = 10;

/// See: https://doc.rust-lang.org/std/sync/struct.Mutex.html
pub fn compute_with_threads_mutex_arc(
    persons: &Vec<Person>,
    work: fn(&[Person]) -> usize,
) -> usize {
    let persons_chuncked: Vec<Vec<Person>> = persons
        .chunks(persons.len() / NUM_THREADS)
        .map(|p| p.to_owned())
        .collect();

    let persons_arc = Arc::new(persons_chuncked);
    let result = Mutex::new(0);

    // Add missing code to compute, using NUM_THREADS number of threads and mutex / arc combination

    result.into_inner().unwrap()
}
/// See: https://doc.rust-lang.org/std/sync/mpsc/index.html
pub fn compute_with_threads_channels(persons: &Vec<Person>, work: fn(&[Person]) -> usize) -> usize {
    let persons_chuncked: Vec<Vec<Person>> = persons
        .chunks(persons.len() / NUM_THREADS)
        .map(|p| p.to_owned())
        .collect();
    let persons_arc = Arc::new(persons_chuncked);

    let (tx, rx) = std::sync::mpsc::channel::<usize>();
    let mut result = 0;

    // Add missing code to compute, using NUM_THREADS number of threads and channels

    result
}

///See: https://doc.rust-lang.org/stable/std/thread/fn.scope.html
pub fn compute_with_threads_scoped(persons: &Vec<Person>, work: fn(&[Person]) -> usize) -> usize {
    let mut persons_chuncked = persons.chunks(persons.len() / NUM_THREADS);

    let result = thread::scope(|s| {
        // Add missing code to compute, using NUM_THREADS number of threads.
        // NB: No mutex or arc should be needed.

        0
    });

    result
}

pub fn get_persons(n: usize) -> Vec<Person> {
    let mut persons = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        persons.push(Person::new(
            "Ola".to_owned(),
            "Nordmann".to_owned(),
            rng.gen_range(0..100),
        ))
    }

    persons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_with_threads_mutex_arc() {
        let persons = get_persons(10_000);

        let work = |collection: &[Person]| collection.iter().map(|p| p.age() as usize).sum();

        let expected: usize = work(&persons);

        let actual = compute_with_threads_mutex_arc(&persons, work);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_compute_with_threads_channels() {
        let persons = get_persons(10_000);

        let work = |collection: &[Person]| collection.iter().map(|p| p.age() as usize).sum();

        let expected: usize = work(&persons);

        let actual = compute_with_threads_channels(&persons, work);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_compute_with_threads_scoped() {
        let persons = get_persons(10_000);

        let work = |collection: &[Person]| collection.iter().map(|p| p.age() as usize).sum();

        let expected: usize = work(&persons);

        let actual = compute_with_threads_scoped(&persons, work);

        assert_eq!(expected, actual);
    }
}
