use std::io::{self, Read, Write};
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

    writeln!(file, "Looks like your car is a {} from {}!", car.model, car.year).unwrap();

}

fn main() {
    reading_from_console();
}