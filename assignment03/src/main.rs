use rand::Rng;

const MAX_VALUE: i32 = 20;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess < secret {
        -1
    } else {
        1
    }
}

fn main() {
    let mut number_of_guesses = 0;
    let mut correct = false;

    let secret = rand::thread_rng().gen_range(0..MAX_VALUE);

    while !correct {
        let guess = rand::thread_rng().gen_range(0..MAX_VALUE);
        let guess_proximity = check_guess(guess, secret);

        if guess_proximity == 0 {
            println!("You guessed {} - the right number!", guess);

            correct = true;
        } else if guess_proximity == -1 {
            println!("You guessed {} - go lower!", guess);
        } else {
            println!("You guessed {} - go higher!", guess);
        }

        number_of_guesses += 1;
    }
    
    println!("It took {} guesses to find the right number!", number_of_guesses);
}
