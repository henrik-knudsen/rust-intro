use std::{
    sync::{Arc, Mutex},
    thread,
};

fn mutex_arc_example() -> usize {
    const N: usize = 5;
    let data_arc = Arc::new(vec![1, 2, 3, 4]);
    let res_mutex = Arc::new(Mutex::new(0));

    let mut handles = Vec::with_capacity(N);

    for _ in 0..N {
        let data_arc_clone = Arc::clone(&data_arc);
        let res_mutex_clone = Arc::clone(&res_mutex);

        handles.push(thread::spawn(move || {
            // This is the result of some important and long-ish work.
            let result = data_arc_clone.iter().fold(0, |acc, x| acc + x * 2);
            *res_mutex_clone.lock().unwrap() += result;
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("The thread creating or execution failed !")
    }

    let value = *res_mutex.lock().unwrap();
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_arc_example() {
        assert_eq!(100, mutex_arc_example())
    }
}
