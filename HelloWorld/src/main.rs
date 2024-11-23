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

/*
use std::fs::OpenOptions;
use std::io::Write;

// echo "Hello World!" > msg.txt
// cat msg.txt
// echo "Hello World 2!" >> msg.txt
// cat msg.txt

fn append_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")
        .unwrap();

    writeln!(file, "This line is appended to the file.").unwrap();
}

fn main() {
    append_to_file();
    println!("Successfully appended to the file.");
}
*/

/*
use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("python3")
        .arg("my_script.py")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    executing_os_commands_linux();
}
*/

/*
enum Pets {
    Dog(String, u8),
    Hamster(String),
}

impl Pets {
    fn introduce_yourself(&self) {
        match &self {
            Pets::Dog(name: &String, age: &u8) => {
                println!("Hey, my name is {}, I am {} years old!", name, age);
            },
            Pets::Hamster(name: &String) => {
                println!("Hey, my name is {}", name);
            },
        }
    }
}


fn main() {
    let pet1: Pets = Pets::Dog(format!("Black", 3);
    let pet2: Pets = Pets::Hamster(format!("Gerbbie"));

    pet1.introduce_yourself();
    pet2.introduce_yourself();
}
*/

/*
use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Write(String, String),
}

impl FileOperation {
    fn get_user_input() -> String {
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let buffer = buffer.trim().to_string();

        return buffer;
    }

    fn validate_file(filename: &String) -> bool {
        Path::new(filename).exists()
    }
}


fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            if !FileOperation::validate_file(&filename) {
                fs::File::create(filename.clone()).unwrap();

                println!("File '{}' created successfully.", filename);
            } else {
                println!("File '{}' could not be created because a file with the same name already exists.", filename);
            }
        }

        FileOperation::Rename(old_name, new_name) => {
            if FileOperation::validate_file(&old_name) && !FileOperation::validate_file(&new_name) {
                fs::rename(old_name.clone(), new_name.clone()).unwrap();

                println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
            } else if FileOperation::validate_file(&old_name) {
                println!("File could not be renamed because a file with the new name already exists!");
            } else {
                println!("File with name {} does not exist! Failed to rename!", old_name);
            } 
        }

        FileOperation::Write(filename, content) => {
            if FileOperation::validate_file(&filename) {
                let mut file = fs::File::create(filename.clone()).unwrap();

                write!(file, "{}", content);
                println!("Successfully edited content of file!");
            } else {
                println!("File with name {} does not exist! Failed to fill!", filename);
            }
        }
    }
}

fn main() {
    for _ in 0..2 {
        println!("Choose an operation:");
        println!("1. Create a new file");
        println!("2. Rename an existing file");
        println!("3. Edit contents of existing file");
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
    
        match choice.trim() {
            "1" => {
                println!("Type a name for the file you want to create:");
                let input = FileOperation::get_user_input();
                perform_operation(FileOperation::Create(input));
                // TODO: Prompt for new filename and call perform_operation
            }
            "2" => {
                println!("Type the name of the file you want to rename:");
                let inputA: String = FileOperation::get_user_input();
                println!("Type the new name for the file you want to rename:");
                let inputB: String = FileOperation::get_user_input();
                perform_operation(FileOperation::Rename(inputA, inputB));
                // TODO: Prompt for old and new filenames and call perform_operation
            }
            "3" => {
                println!("Type the name of the file you want to edit:");
                let inputA: String = FileOperation::get_user_input();
                println!("Type the contents of the file:");
                let inputB: String = FileOperation::get_user_input();
                perform_operation(FileOperation::Write(inputA, inputB));
            }
            _ => println!("Invalid choice"),
        }
    }
}
*/

/*
fn main() {
    let new_file: &str = "rustandlinux.txt";
    let command_to_execute: String = format!("touch {}", new_file);
    println!("{}", command_to_execute);

    let command = Command::new{program: "sh"}.arg("-c").arg(command_to_execute).spawn();

}
*/

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn panic_examples() {
    println!("Everything is good");
    // panic!("Crash the program, stop running, clean the memory");
    println!("This won't be printed.");
    let v = vec![1, 2, 3];
    println!("{:?}", v[99]); // This will cause a panic because the index is out of bounds
}

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    println!("Hello");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}*/
/*
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let f = File::open("my_protected_folder/exam.txt");
    println!("{:?}", f);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("my_protected_folder/exam.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}*/

/*
use std::fs::File;
use std::io::Read;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2ver() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn read_username_from_file_3ver() -> Result<String, io::Error> {
    let mut s: String = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let username2 = read_username_from_file_2ver().unwrap();
    let username3 = read_username_from_file_3ver().unwrap();
    println!("{:?}{:?}", username2, username3);
}*/

/*
pub trait Comparison {
    fn get_comparable_number(&self) -> f32;
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Item {
    price: i32,
}

impl Item {
    fn new(new_price: i32) -> Item {
        Item {
            price: new_price,
        }
    }
}

impl Comparison for Item {
    fn get_comparable_number(&self) -> f32 {
        self.price as f32
    }
}

impl Comparison for i32 {
    fn get_comparable_number(&self) -> f32 {
        *self as f32
    }
}

impl Comparison for f32 {
    fn get_comparable_number(&self) -> f32 {
        *self
    }
}

fn largest_int(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    b
}

fn largest_double(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    }

    b
}

fn largest_item(item1: Item, item2: Item) -> Item {
    if item1 > item2 {
        return item1;
    }

    item2
}

fn largest_universal<T: Comparison> (a: T, b: T) -> T {
    if a.get_comparable_number() > b.get_comparable_number() {
        return a;
    }

    b
}


fn main() {
    let item1: Item = Item::new(100);
    let item2: Item = Item::new(1000);

    let item3: Item = Item::new(100);
    let item4: Item = Item::new(1000);

    println!("{}", largest_int(10, 50));
    println!("{}", largest_double(1.1, 5.5));
    println!("{:?}", largest_item(item1, item2));
    println!("{:?}", largest_universal(10, 50));
    println!("{:?}", largest_universal(1.1, 5.5));
    println!("{:?}", largest_universal(item3, item4));
}*/

/*
#[derive(Debug)]
struct Person<U, T> {
    name: U,
    age: T,
}

impl<U, T> Person<U, T> {
    fn new(name: U, age: T) -> Person<U, T> {
        Person {
            name: name,
            age: age,
        }
    }

    fn add(&mut self, extra: T) {
        self.age = extra;
    }
}

fn main() {
    let mut p1 = Person::new("John", 5);
    p1.add(7);

    let mut p2 = Person::new("Bipper", "six".to_string());
    p2.add("bipper bopper".to_string());

    println!("{:?}", p1);
    println!("{:?}", p2);
}*/

/*
pub trait Talkative {
    fn talk(&self);
}

pub struct Person {
    name: String,
}

pub struct Parent {
    name: String,
    child: Person,
}

impl Person {
    fn new(name: String) -> Person {
        Person {
            name: name,
        }
    }
}

impl Parent {
    fn new(name: String, child: Person) -> Parent {
        Parent {
            name: name,
            child: child,
        }
    }
}

impl Talkative for Person {
    fn talk(&self) {
        println!("Hi my name is {}!", self.name);
    }
}

impl Talkative for Parent {
    fn talk(&self) {
        println!("Hi my name is {} and this is my child...", self.name);

        self.child.talk();
    }
}

fn main() {
    let child: Person = Person::new("Devi".to_string());
    let parent: Parent = Parent::new("Mother".to_string(), child);

    let new_vec: Vec<dyn Talkative> = Vec::new(child, parent);

    for idx in new_vec.iter() {
        new_vec[idx].talk();
    }

    parent.talk();
}*/

/*
fn same_method_same_logical_entity(){

    // this is a big idea.
    // bind different data types with same behaviour
    // as one logical unit
    pub trait AreaInfo {
        fn get_area(&self) -> f64;
        fn say_hi(&self);
    }
    

    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
    }

    impl AreaInfo for Rectangle {
        fn get_area(&self) -> f64 {
            self.width * self.height
        }

        fn say_hi(&self) {
            println!("Hi!");
        }
    }


    pub struct Circle {
        pub radius: f64,
    }

    impl AreaInfo for Circle {
        fn get_area(&self) -> f64 {
            self.radius * self.radius * 3.14 as f64
        }

        fn say_hi(&self) {
            println!("Hi!");
        }
    }

    // You could say, it's almost the same, well what is same for you is not the same for the compiler.

    // Make sure nothing is broken

    let rec = Rectangle {width:5.0,height:8.0};
    let circle = Circle {radius: 5.0};

    println!("Area of a rectangle {}", rec.get_area());
    println!("Area of a circle {}", circle.get_area());

    // And now the Magic or cheating, I don't know how to call it
    
    let shapes: Vec<&dyn AreaInfo> = vec![&rec,&circle];

    // dyn -> dynamic keyword 
    // https://doc.rust-lang.org/std/keyword.dyn.html

    // Dynamically dispatch the type of object
    for shape in shapes.iter() {
        println!("{}", shape.get_area());
        shape.say_hi();
    }
}

fn main() {
    same_method_same_logical_entity();
}*/

/*
fn benefits_of_logical_entity(){
        
    pub trait Summary { // Trait should be public if we want to allow others to implement it
        fn summarize(&self) -> String; // no body just declaration like interface
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle { 
        fn summarize(&self) -> String { 
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    let tw = Tweet {
             username: String::from("Elon"),
             content: String::from("to the Moon"),
             reply: false,
             retweet: false,
         };
    println!("{}",tw.summarize());
        
    let article = NewsArticle {
             headline: String::from("Elon sells Bitcoin"),
             location: String::from("Menlo Park, CA, USA"),
             author: String::from("CNN"),
             content: String::from("FBI investigates"),
         };
    
    println!("{}", article.summarize());

    // Real reason we need to use traits or interfaces
    // Change you coding paradigm, start to programm to behavior!

    pub fn notify_sugar_syntax(item: impl Summary) { // your function will accept any parameter that implements Summary trait. (so all parameters will have the common behavior)
        println!("Breaking news! {}", item.summarize());
    }

    // Same logic as above but explicit, this is refereed to the idea TRAIT BOUNDS
    // or in simple language, sometimes you want to accept parameters, which implement more than one trait(have more than one common method to call on it)
    
    pub fn notify_real_syntax<T: Summary>(item: T){ // please notice generics you are saying. My function will accept as a parameter a generic type T which implements Summary trait, because I just want to make sure that I can call the methods safely.
        
        println!("Breaking news! {}", item.summarize());
    }


    notify_real_syntax(tw);
    notify_sugar_syntax(article);

}

fn main() {
    benefits_of_logical_entity();
}*/

/*
fn last_lecture_problem_fixing(){
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { 
        let mut largest = list[0]; // we need Copy trait to achieve that operation
        
        for &item in list.iter() {
            if item > largest { // we need PartialOrd trait to be able to compare elements
                largest = item;
            }
        }
        largest
    }

    // where clause
    fn largest_2<T>(list: &[T]) -> T 
        where T: PartialOrd + Copy,
        {
        let mut largest = list[0]; // we need Copy trait to achieve that operation
        
        for &item in list.iter() {
            if item > largest { // we need PartialOrd trait to be able to compare elements
                largest = item;
            }
        }
        largest
    }


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_2(&char_list);
    println!("The largest char is {}", result);
}
*/

/*
hw for generics and traits

three structs: gold bitcoin smp500 (check this; will be on blackboard)
implement trait PriceItem for all 3
make api call for prices within trait (can hardcode; but no)
make a vector to store one of each
iterate every 10 seconds
print as a string IN DOLLARS
*/

/*
// Define our data structure
struct Data {
    value: i32,
}

// Higher-order function: defines what needs to be done
fn process_data(data: &mut [Data], operation: fn(&mut Data)) {
    for item in data.iter_mut() {
        operation(item);
    }
}

// Specific operations: actual functions which do the work
fn double_value(data: &mut Data) {
    data.value *= 2;
}

fn square_value(data: &mut Data) {
    data.value *= data.value;
}

// Helper function to print values without closures
fn print_values(items: &[Data]) {
    print!("Values: ");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", item.value);
    }
    println!();
}

fn main() {
    let mut items = vec![
        Data { value: 1 },
        Data { value: 2 },
        Data { value: 3 },
        Data { value: 4 },
        Data { value: 5 },
    ];
    
    // The specific operation is decided here
    print!("Original ");
    print_values(&items);
    
    process_data(&mut items, double_value);
    print!("After doubling: ");
    print_values(&items);
    
    // We can easily switch to a different operation
    process_data(&mut items, square_value);
    print!("After squaring: ");
    print_values(&items);
}
*/

/*
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

struct Calculator {
    addition: fn(i32, i32) -> i32,
    subtraction: fn(i32, i32) -> i32,
    multiplication: fn(i32, i32) -> i32,
}

impl Calculator {
    fn new(addition_behavior: fn(i32, i32) -> i32, subtraction_behavior: fn(i32, i32) -> i32, multiplication_behavior: fn(i32, i32) -> i32) -> Calculator {
        Calculator {
            addition: addition_behavior,
            subtraction: subtraction_behavior,
            multiplication: multiplication_behavior,
        }
    }

    fn add(&self, x: i32, y: i32) -> i32 {
        (self.addition)(x, y)
    }

    fn sub(&self, x: i32, y: i32) -> i32 {
        (self.subtraction)(x, y)
    }
    
    fn mul(&self, x: i32, y: i32) -> i32 {
        (self.multiplication)(x, y)
    }
}


fn main() {
    let c = Calculator::new(add, sub, mul);
    let result_a = c.add(5, 10);
    let result_b = c.sub(10, 5);
    let result_c = c.mul(10, 10);

    println!("{}, {}, {}", result_a, result_b, result_c);
}*/

/*
fn essence_example_closure() {
    let x = 21;
    let mut message = "hello from closure".to_string();
    let get_answer = |y: i32| x + y;
    let get_answer2 = |y: i32| {
        message.push_str("does the magic work");
        println!("{}", message);
        x + y
    };
    // fn
    // fn mut
    // fn once
    
    println!("{:?}", get_answer(21));  // Output: 42
    println!("{:?}", get_answer2(21));  // Output: 42

    println!("{:?}", message);  // Output: 42
}

fn main() {
    essence_example_closure();
}*/

/*
fn using_function_as_variable() {
    // Regular function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Function pointer
    let f = add;

    // Closure with explicit types
    let f = |x: i32, y: i32| { x + y };

    // Simplified closure
    let f = |x: i32, y: i32| x + y;

    // Closure with inferred types
    let f = |x, y| x + y;
    
    let result = f(1, 2);
    println!("{}", result);  // Output: 3
}

fn using_function_as_parameter() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
        let result = operation(x, y);
        println!("Result of operation: {}", result);    
    }

    calculator(1, 2, add);
    calculator(1, 2, |x, y| x + y);
    calculator(1, 2, |x, y| x - y);
    calculator(1, 2, |x, y| x * y);
}
*/

/*
use std::ops::Add;

fn f<T, U>(x:T, y:U) -> T {
    where T:Add<Output>, U:Add{
        x + y as T
    }
}*/

/*
fn main() {
    using_function_as_variable();
    using_function_as_parameter();
    let result1 = f(1,2.0);
    println!("{}", result1);
    let result2 = f("1".to_string(), "2");
    println!("{}", result2);
}*/

/*
fn using_function_as_parameter() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
        let result = operation(x, y);
        println!("result of operation: {}", result);
    }

    calculator(1, 2, add);
    calculator(1, 2, |x, y| x + y);
    calculator(1, 2, |x, y| x - y);
    calculator(1, 2, |x, y| x * y);
}

fn main() {
    using_function_as_parameter();
}*/

/*
fn box_pointer_intro() {
    let box_default = Box::new(100);
    println!("{}", box_default);  // Output: 100
}

fn main() {
    box_pointer_intro();
}*/
/*
fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    
    #[derive(Debug)]
    struct Dog;
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }
}

fn main() {
    box_polymorphism();
}*/

use std::rc::Rc;
use std::cell::RefCell;

fn reference_counting_simple() {
    let num = 10;
    let num_in_heap = Rc::new(num);

    let _copy2_of_num = Rc::clone(&num_in_heap);
    let _copy3_of_num = Rc::clone(&num_in_heap);
    let _copy4_of_num = Rc::clone(&num_in_heap);

    println!("num in heap has: {} references", 
             Rc::strong_count(&num_in_heap));
}

struct FamilyMember {
    tv: Rc<TV>,
}

struct TV;

fn sharing_resource_rc_count() {
    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(TV);
        FamilyMember {
            tv: Rc::clone(&tv_is_on),
        }
    }

    let dad = member_start_watch_tv();
    println!("How many people watching {}", Rc::strong_count(&dad.tv));

    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("How many people watching {}", Rc::strong_count(&dad.tv));

    let me = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("How many people watching {}", Rc::strong_count(&me.tv));

    drop(dad);
    drop(me);

    println!("How many people watching {}", Rc::strong_count(&mom.tv));
}

fn ref_cell_simple() {
    let num = 10;
    let data = RefCell::new(num);
    
    // Borrow the data immutably
    let data_ref = data.borrow();
    println!("Data: {}", data_ref);

    // Drop the immutable borrow so we can borrow mutably
    drop(data_ref);

    println!("Data: {:?}", data);

    // Borrow the data mutably
    let mut data_mut = data.borrow_mut();
    *data_mut += 1;
    println!("Data: {}", data_mut);
}

fn interior_mutability() {
    #[derive(Debug)]
    struct MyData {
        data: f64
    }

    let base: Rc<RefCell<MyData>> = Rc::new(RefCell::new(
        MyData {
            data: 70.00
        }
    ));

    println!("base: {:?}", base);
    
    {
        let mut base_2 = base.borrow_mut();
        base_2.data -= 10.00;
        println!("base_2: {:?}", base_2);
    }
 
    println!("base: {:?}", base);
 
    let mut base_3 = base.borrow_mut();
    base_3.data += 30.00;
 
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}

fn sharing_resource_refcell_count() {
    struct FamilyMember {
        tv: Rc<RefCell<TV>>,
    }

    #[derive(Debug)]
    struct TV {
        channel: String,
    }

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(RefCell::new(TV{channel:"BBC".to_string()}));
        
        FamilyMember {
            tv: tv_is_on, 
        }
    }

    let dad = member_start_watch_tv();
    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("TV channel for mom {:?}", mom.tv);

    let mut remote_control = dad.tv.borrow_mut();
    println!("TV channel {:?}", remote_control);

    remote_control.channel = "MTV".to_string();
    println!("TV channel {:?}", remote_control);
    drop(remote_control);
    println!("TV channel for mom {:?}", mom.tv);
}

fn joint_bank_account_example() {
    #[derive(Debug)]
    struct BankAccount {
        balance: RefCell<f64>,
    }
    
    impl BankAccount {
        fn new(initial_balance: f64) -> Rc<Self> {
            Rc::new(BankAccount {
                balance: RefCell::new(initial_balance),
            })
        }
    
        fn deposit(&self, amount: f64) {
            let mut balance = self.balance.borrow_mut();
            *balance += amount;
            println!("Deposited ${:.2}, new balance: ${:.2}", amount, *balance);
        }
    
        fn withdraw(&self, amount: f64) {
            let mut balance = self.balance.borrow_mut();
            if *balance >= amount {
                *balance -= amount;
                println!("Withdrew ${:.2}, new balance: ${:.2}", amount, *balance);
            } else {
                println!("Insufficient funds. Current balance: ${:.2}", *balance);
            }
        }
    }
    
    let account = BankAccount::new(1000.0);
    let joint_account = Rc::clone(&account);

    account.deposit(500.0);
    joint_account.withdraw(200.0);
    account.withdraw(1500.0);
}

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person {
            name: name,
            age: age,
        }
    }
}

fn main() {
    let p = Person::new("Devi".to_string(), 23);
    let watchers: i32 = 0;


    println!("reference counting simple");
    reference_counting_simple();
    println!("\nsharing resource rc count");
    sharing_resource_rc_count();
    println!("\nref cell simple");
    ref_cell_simple();
    println!("\ninterior mutability");
    interior_mutability();
    println!("\nsharing resource refcell count");
    sharing_resource_refcell_count();
    println!("\njoint bank account example");
    joint_bank_account_example();
}