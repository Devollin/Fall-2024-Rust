fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    const TOTAL_NUMBERS: usize = 10;
    let numbers: [i32; TOTAL_NUMBERS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for index in numbers.iter() {
        let even = is_even(*index);
        let display_text = if even {
            "even"
        } else {
            "odd"
        };

        println!("{} is {}.", index, display_text);
    }

    for index in numbers.iter() {
        let divisible_by_five = index % 5 == 0;
        let divisible_by_three = index % 3 == 0;
        let result = if divisible_by_five & divisible_by_three {
            "FizzBuzz"
        } else if divisible_by_five {
           "Buzz"
        } else if divisible_by_three {
            "Fizz"
        } else {
            "NULL"
        };

        if result != "NULL" {
            println!("{}: {}", index, result);
        }
    }

    let mut counter = 0;
    let mut sum = 0;
    let mut largest_number = 0;

    while counter != TOTAL_NUMBERS {
        sum += numbers[counter];

        if numbers[counter] > largest_number {
            largest_number = numbers[counter];
        }

        counter += 1;
    }

    println!("Sum of all numbers: {}, largest number: {}", sum, largest_number);
}
