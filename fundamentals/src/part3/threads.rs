use std::thread;

fn thread_example() {
    let handler = thread::spawn(|| {
        // thread code
    });

    handler.join().unwrap();
}

fn thread_multiple_example() {
    let n = 10;
    let mut handles = Vec::with_capacity(n);

    for i in 0..n {
        let handle = thread::spawn(|| {
            // thread code
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn scoped_thread_example() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}
