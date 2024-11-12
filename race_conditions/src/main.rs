use std::thread;
use std::sync::Arc;

fn main() {
    let counter = Arc::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut count = *counter;
            count += 1;
            println!("Thread incremented counter to: {}", count);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter);
}
