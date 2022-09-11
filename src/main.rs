mod rps;
mod score;

use rps::RockPaperScissors;
use score::Score;

use rand::prelude::*;
use std::io::{self, Write};

fn prompt_input(prompt: &str) -> String {
    // Start out by printing prompt
    print!("{}", prompt);
    // flush() is used to make sure that the prompt is printed before user input
    io::stdout().flush().expect("Error: Flush failed!");

    // Finally get and return user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Read failed!");
    return input.trim().to_string();
}

fn get_input(prompt: &str) -> RockPaperScissors {
    let inp = prompt_input(prompt);

    return match inp.to_lowercase().as_str() {
        "r" | "rock" => RockPaperScissors::Rock,
        "p" | "paper" => RockPaperScissors::Paper,
        "s" | "scissors" => RockPaperScissors::Scissors,
        _ => RockPaperScissors::Invalid,
    };
}

fn computer_guess() -> RockPaperScissors {
    let mut rng = rand::thread_rng();

    return match rng.gen_range(0..=2) {
        0 => RockPaperScissors::Rock,
        1 => RockPaperScissors::Paper,
        _ => RockPaperScissors::Scissors,
    };
}

fn main() {
    // Initiate statuses
    let mut score = Score::new();

    println!("Welcome to Rock Paper Scissors!");

    loop {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        // Print game score and status
        println!("Current score is:\n{}", score.to_string());

        // Get player input
        let player_choice = get_input("Rock, Paper or Scissors? ");

        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        match player_choice {
            // Invalid input
            RockPaperScissors::Invalid => {
                println!("Invalid input!");
            },
            // Valid input
            _ => {
                // Get computer guess
                let computer_choice = computer_guess();
                
                // Print guesses
                println!("You chose: {}", player_choice.to_string());
                println!("Computer chose: {}", computer_choice.to_string());

                // If player wins, increment player_score
                if player_choice.beats(&computer_choice) {
                    println!("You win!");
                    score.incr_player();
                } else if computer_choice.beats(&player_choice) {
                    println!("You lose!");
                    score.incr_comp();
                // If draw, increment draw_count
                } else {
                    println!("Draw!");
                    score.incr_draw();
                }
            }
        }

        // Prompt for next game
        if prompt_input("Wanna continue? (Y/n) ") == "n" {
            println!("Bye!");
            break;
        }
    }
}
