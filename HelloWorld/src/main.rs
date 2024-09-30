/*
fn sum(total: &mut i32, low: i32, high: i32) {
    for index in low..=high {
        *total += index;
    }
}

fn main() {
    let mut total = 0;

    sum(&mut total, 0, 100);

    println!("Sum from 0 to 100 is: {}", total);
}

fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    for num in (low..high).step_by(step as usize) {
        *total += num;
    }
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word: &str = "";
    let mut max_count = 0;

    for idx in 0..words.len() {
        let mut current_count = 0;
        
        for index in 0..words.len() {
            if words[index] == words[idx] {
                current_count += 1;
            }
        }

        if current_count > max_count {
            max_word = words[idx];
            max_count = current_count;
        }
    }

    ((*max_word).to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
*/

/*
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let number = x.to_string();

    return number.reverse() == number;
}

fn main() {
    println!("{}", is_palindrome(32).to_string());
    println!("{}", is_palindrome(121).to_string());
    println!("{}", is_palindrome(-10).to_string());
}
*/

/*
struct Computer {
    cpu: String,
    ram: i32,
}

impl Computer {
    fn do_thing(&self) -> String {
        self.cpu.clone()
    }

    fn do_other_thing(&self) -> i32 {
        self.ram.clone()
    }

    fn do_other_other_thing(&self) -> String {
        "test".to_string()
    }
}

fn main() {
    let stuff = Computer {
        cpu: "Ryzen 5".to_string(),
        ram: 10,
    };

    let mut other_stuff = Computer {
        cpu: "Ryzen 7".to_string(),
        ram: 100,
    };

    other_stuff.cpu = "Ryzen 5.5".to_string();

    println!("{}, {} GB RAM, running as \"{}\"", stuff.do_thing(), stuff.do_other_thing(), stuff.do_other_other_thing());
    println!("{}, {} GB RAM, running as \"{}\"", other_stuff.do_thing(), other_stuff.do_other_thing(), other_stuff.do_other_other_thing());
}
*/

/*
use std::arch::asm;

fn main() {
    let message = b"Hello, direct syscall!\n";

    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}
*/

/*
use std::fs::File;
use std::io::Write;

fn create_and_write_to_file() {
    let mut file = File::create("example.txt").unwrap();
    let file1 = File::create("example1.txt").unwrap();

    println!("{:?}", file1);
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
}

fn main() {
    create_and_write_to_file();
    println!("File created and written successfully.");
}
*/

/*
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read_entire_file() {
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // loads entire file into variable
    println!("File contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    println!("Reading entire file:");
    read_entire_file();

    println!("\nReading file line by line:");
    read_file_line_by_line();
}
*/