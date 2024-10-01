use std::io::{self, BufReader, BufRead, Write};
use std::fs::File;

struct Car {
    model: String,
    year: u32,
}

fn reading_from_console() {
    let mut file = File::create("car_info.txt").unwrap();
    let mut buffer = String::new();

    print!("What's the model of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What year is the car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    let car = Car { model, year };
    println!("Looks like your car is a {} from {}!\nThis info has been saved to car_info.txt!", car.model, car.year);

    writeln!(file, "{}", car.model).unwrap();
    writeln!(file, "{}", car.year).unwrap();
}

fn read_from_file() {
    let file = File::open("car_info.txt").unwrap();
    let reader = BufReader::new(file);
    let mut index = 0;

    println!("Reading from file...");

    for line in reader.lines() {
        index += 1;

        if index == 1 {
            println!("Model: {}", line.unwrap());
        } else if index == 2 {
            println!("Year: {}", line.unwrap());
        }
    }
}

fn main() {
    reading_from_console();
    read_from_file();
}