use rand::prelude::*;
use std::{io, io::prelude::*, process::exit, path::Path, fs, fs::File};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
struct Data {
    correct: i32,
    incorrect: i32,
}

fn main() {
    clear_console();

    let path = !Path::new("data.json").exists();

    if path {
        generate_json_file();
    }

    let mut guess = String::new();

    println!("Guess a number between: {}", "1-10".yellow());
    io::stdin().read_line(&mut guess).expect("Error");

    if guess.trim().is_empty() {
        println!("You need to provide a guess!");
        main();
    } else {
        generate_random_number(guess);
    }

}

fn generate_random_number(guess: String) ->  std::io::Result<()> {
    clear_console();

    let file = File::open("data.json").unwrap();

    let v: Data = serde_json::from_reader(file)?;

    let mut rng = thread_rng();
    let x: u8 = rng.gen_range(1..10);

    let guess: u8 = guess.trim().parse().unwrap();
    if guess == x {
        println!("Your guess: {} was {}\n", guess, "correct!".green());
        let correct = v.correct;
        let count_correct = correct + 1;
        let count_incorrect = v.incorrect;
        write_correct_to_json(count_correct, count_incorrect);
    } else {
        println!("Your guess: {} was {}\nIt was: {}\n", guess, "incorrect!".red(), x);
        let incorrect = v.incorrect;
        let count_incorrect = incorrect + 1;
        let count_correct = v.correct;
        write_incorrect_to_json(count_correct ,count_incorrect);
    }

    exit_or_not();
    Ok(())
}

fn exit_or_not() {
    println!("Do you want to play a new round, exit or view your statistics?");
    println!("For a new round type '{}', to exit type '{}', to view statistics type '{}'", "1".yellow(), "2".yellow(), "3".yellow());

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Error");

    if answer.trim().is_empty() {

        //TODO - Add check what type answer is. If not u8 then go back to exit_or_not;
        
        clear_console();
        println!("{}\n", "You need to provide an answer!".red());
        exit_or_not();
    }

    let answer: u8 = answer.trim().parse().unwrap();

    if answer == 1 {
        main();
    } else if answer == 2 {
        clear_console();
        exit(0)
    } else if answer == 3 {
        clear_console();
        show_statistics();
    } else {
        clear_console();
        println!("{}\n", "Not a valid option!".red());
        exit_or_not();
    }
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn generate_json_file() ->  std::io::Result<()> {
    let count_correct: i32 = 0;
    let count_incorrect: i32 = 0;

    //let data = Data {count_correct, count_incorrect};

    let data = Data {
        correct: count_correct,
        incorrect: count_incorrect,
    };

    let serialized = serde_json::to_string(&data).unwrap();

    File::create("data.json")?;
    fs::write("data.json", serialized)?;
    Ok(())
    
    //let mut file = File::create("data.json")?;
    //file.write(serialized)?;

}

fn write_correct_to_json(count_correct: i32, count_incorrect: i32) -> std::io::Result<()> {

    let data = Data {
        correct: count_correct,
        incorrect: count_incorrect,
    };

    let serialized = serde_json::to_string(&data).unwrap();

    File::create("data.json")?;
    fs::write("data.json", serialized)?;

    Ok(())
}

fn write_incorrect_to_json(count_correct: i32, count_incorrect: i32) -> std::io::Result<()> {

    let data = Data {
        correct: count_correct,
        incorrect: count_incorrect,
    };

    let serialized = serde_json::to_string(&data).unwrap();

    File::create("data.json")?;
    fs::write("data.json", serialized)?;

    Ok(())
}

fn show_statistics() -> Result<()> {
    let file = File::open("data.json").unwrap();

    let v: Data = serde_json::from_reader(file)?;

    println!("You guessed: {} {} and {} {}\n", v.correct,"correct".green(), v.incorrect,"incorrect".red());

    use std::process::Command;
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

    main();
    Ok(())
}