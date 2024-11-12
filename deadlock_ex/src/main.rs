use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create two mutexes
    let mutex_a = Arc::new(Mutex::new(0));
    let mutex_b = Arc::new(Mutex::new(0));

    // Clone the Arc pointers for each thread
    let mutex_a1 = Arc::clone(&mutex_a);
    let mutex_b1 = Arc::clone(&mutex_b);

    // First thread
    let thread1 = thread::spawn(move || {
        let _lock_a = mutex_a1.lock().unwrap();
        println!("Thread 1 locked mutex_a");

        // sleep to cause deadlock
        thread::sleep(Duration::from_millis(50));

        let _lock_b = mutex_b1.lock().unwrap();
        println!("Thread 1 locked mutex_b");
    });

    // Second thread
    let mutex_a2 = Arc::clone(&mutex_a);
    let mutex_b2 = Arc::clone(&mutex_b);

    let thread2 = thread::spawn(move || {
        let _lock_b = mutex_b2.lock().unwrap();
        println!("Thread 2 locked mutex_b");

        // sleep to cause deadlock
        thread::sleep(Duration::from_millis(50));

        let _lock_a = mutex_a2.lock().unwrap();
        println!("Thread 2 locked mutex_a");
    });

    // D E A D L O C K
    let _ = thread1.join();
    let _ = thread2.join();

    println!("This will not print");
}
