use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};


fn thread_joining() {
    let mut threads = vec![];

    for num in 1..4 {
        let thread = thread::spawn(move || {
            let duration = rand::thread_rng().gen_range(1..5);
            println!("Thread {} started! Waiting for {} seconds.", num, duration);
            
            thread::sleep(Duration::from_secs(duration));

            println!("Thread {} finished!", num);
        });

        threads.push(thread);
    };

    for thread in threads {
        thread.join().unwrap();
    };

    println!("All threads completed!");
}


fn shared_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..5 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let thread = thread::spawn(move || {
            for _ in 0..10 {
                *counter.lock().unwrap() += 1;
            };
        });

        threads.push(thread);
    };

    for thread in threads {
        thread.join().unwrap();
    };

    println!("Final count: {:?}", counter.lock().unwrap());
}


fn main() {
    println!("Assignment Part 1:");

    thread_joining();

    println!("\nAssignment Part 2:");

    shared_counter();
}
