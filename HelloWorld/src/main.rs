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