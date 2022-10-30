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

    let mut handles = Vec::with_capacity(NUM_THREADS);
    let result = Mutex::new(0);

    for i in 0..NUM_THREADS {
        let persons_arc_copy = Arc::clone(&persons_arc);

        let handle = thread::spawn(move || work(&persons_arc_copy[i]));
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(value) => *result.lock().unwrap() += value,
            Err(e) => eprintln!("{e:?}"),
        }
    }

    result.into_inner().unwrap()
}
/// See: https://doc.rust-lang.org/std/sync/mpsc/index.html
pub fn compute_with_threads_channels(persons: &Vec<Person>, work: fn(&[Person]) -> usize) -> usize {
    let persons_chuncked: Vec<Vec<Person>> = persons
        .chunks(persons.len() / NUM_THREADS)
        .map(|p| p.to_owned())
        .collect();
    let persons_arc = Arc::new(persons_chuncked);

    let (tx, rx) = std::sync::mpsc::channel();
    let mut result = 0;

    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        let persons_arc_copy = Arc::clone(&persons_arc);

        thread::spawn(move || tx.send(work(&persons_arc_copy[i])).unwrap());
    }

    for _ in 0..NUM_THREADS {
        let intermediate = rx.recv().unwrap();
        result += intermediate;
    }

    result
}

///See: https://doc.rust-lang.org/stable/std/thread/fn.scope.html
pub fn compute_with_threads_scoped(persons: &Vec<Person>, work: fn(&[Person]) -> usize) -> usize {
    let mut persons_chuncked = persons.chunks(persons.len() / NUM_THREADS);

    let result = thread::scope(|s| {
        let mut handles = Vec::with_capacity(NUM_THREADS);
        let mut result = 0;

        for i in 0..NUM_THREADS {
            let chunk = persons_chuncked.next().unwrap();
            let handle = s.spawn(move || work(chunk));
            handles.push(handle);
        }

        for handle in handles {
            match handle.join() {
                Ok(value) => result += value,
                Err(e) => eprintln!("{e:?}"),
            }
        }
        result
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
