use rand::prelude::*;
use std::{io, process::exit};

fn main() {
    clear_console();

    let mut guess = String::new();

    println!("Guess a number between 1-10");
    io::stdin().read_line(&mut guess).expect("Error");

    if guess.trim().is_empty() {
        println!("You need to provide a guess!");
        main();
    } else {
        generate_random_number(guess);
    }
}

fn generate_random_number(guess: String) {
    clear_console();

    let mut rng = thread_rng();
    let x: u8 = rng.gen_range(1..10);

    let guess: u8 = guess.trim().parse().unwrap();
    if guess == x {
        println!("Your guess: {} was correct!\n", guess);
    } else {
        println!("Your guess: {} was incorrect! \nIt was: {}\n", guess, x);
    }
    exit_or_not();
}

fn exit_or_not() {
    println!("Do you want to play a new round or exit?");
    println!("For a new round type '1', to exit type '2'");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Error");

    if answer.trim().is_empty() {
        //TODO - Add check what type answer is. If not u8 then go back to exit_or_not;
        clear_console();
        println!("You need to provide an answer!\n");
        exit_or_not();
    }

    let answer: u8 = answer.trim().parse().unwrap();

    if answer == 1 {
        main();
    } else if answer == 2 {
        clear_console();
        exit(0)
    } else {
        clear_console();
        println!("Not a valid option!\n");
        exit_or_not();
    }
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}