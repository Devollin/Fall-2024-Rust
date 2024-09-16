fn fahrenheit_to_celcius(f: f64) -> f64 {
    return (f - 32.0) * (5.0 / 9.0);
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    return (c * (9.0 / 5.0)) + 32.0;
}

fn main() {
    let mut temperature = 35.0; // in fahrenheit, to start
    let mut counter = 0;

    println!("Temperature of {} degrees fahrenheit is {} degrees celcius.", temperature, fahrenheit_to_celcius(temperature));

    loop {
        if counter == 5 {
            break;
        }

        counter += 1;
        temperature += 1.0;

        println!("Temperature of {} degrees fahrenheit is {} degrees celcius.", temperature, fahrenheit_to_celcius(temperature));
    }
}
