fn assignment1() {
    let operation = |a: i32, b: i32| {
        a * b
    };

    println!("Result: {}", operation(10, 5));
}

fn assignment2() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("{}", tracker);
    };

    update();
    update();
}

fn assignment3() {
    fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        vec.into_iter().map(f).collect()
    }

    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
        // Implement: multiply each number by 2
    });

    let replaced = process_vector(numbers, |x| {
        match x > 2 {
            true => {0}
            _ => {x}
         }
        // Implement: if number > 2, replace with 0, else keep number
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}

fn assignment4() {
    use std::{thread, time::Duration};

    struct ComputeCache<T>
    where
        T: Fn() -> String,
    {
        computation: T,
        // Add fields here
    }

    impl<T> ComputeCache<T>
    where
        T: Fn() -> String,
    {
        fn new(computation: T) -> Self {
            ComputeCache {
                computation: computation,
            }
        }

        fn get_result(&mut self) -> String {
            (self.computation)()
        }
    }

    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}

fn main() {
    let mut index = 0;
    let functions_to_run = [assignment1, assignment2, assignment3, assignment4];

    for func in functions_to_run.iter() {
        index += 1;
        println!("\nAssignment {}", index);
        func();
    }
}