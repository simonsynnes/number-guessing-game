use std::io;
use rand::Rng;

const MAX_ATTEMPTS: u32 = 5;

fn generate_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn read_user_input() -> u32 {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read");

   user_input.trim().parse().expect("Please type a number!")
}

fn compare_numbers(user_input: u32, random_number: u32) -> String {
     if user_input < random_number {
         return String::from("Your guess is too low!");
    } else if user_input > random_number {
        return String::from("Your guess is too high!");
    } else {
        return String::from("You got it correct! :)");
    }
}
fn main() {
    println!("Number guessing game");
    let random_number = generate_random_number();
    let mut guess_attempts = 0;
    println!("Correct number spoiler: {}", random_number);
    loop {
        let user_input = read_user_input();
        let compare_result = compare_numbers(user_input, random_number);
        println!("The user input is: {}", user_input);
        println!("Compare result {}", compare_result);

        guess_attempts += 1;

        if guess_attempts == MAX_ATTEMPTS {
            println!("You ran out of guesses! The answer was: {}", random_number);
            break;
        }

        if user_input == random_number {
            break;
        }
    }

}
