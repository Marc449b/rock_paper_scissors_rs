use rand::prelude::*;
use std::io::{self, Write};

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
    Quit,
    Invalid,
}

impl RockPaperScissors {
    fn beats(&self, other: &RockPaperScissors) -> bool {
        return match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Paper) |
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => true,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) |
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => false,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) |
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => true,
            _ => false,
        };
    }

    fn to_string(&self) -> &'static str {
        return match self {
            RockPaperScissors::Rock => "Rock",
            RockPaperScissors::Paper => "Paper",
            RockPaperScissors::Scissors => "Scissors",
            RockPaperScissors::Quit => "Quit",
            RockPaperScissors::Invalid => "Invalid",
        };
    }
}

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
        "q" | "quit" => RockPaperScissors::Quit,
        _ => RockPaperScissors::Invalid,
    };
}

fn computer_guess() -> RockPaperScissors {
    let mut rng = rand::thread_rng();

    return match rng.gen_range(0..2) {
        0 => RockPaperScissors::Rock,
        1 => RockPaperScissors::Paper,
        _ => RockPaperScissors::Scissors,
    };
}

fn main() {
    let mut player_score = 0;
    let mut draw_count = 0;
    let mut game_count = 0;

    println!("Welcome to Rock Paper Scissors!");

    loop {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        println!("Current score is: Player: {}\nComputer: {}\nDraws: {}\nGamecount: {}\n", player_score, game_count - player_score - draw_count, draw_count, game_count);

        let player_choice = get_input("Rock, Paper or Scissors? quit for quit: ");

        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        match player_choice {
            RockPaperScissors::Quit => {
                println!("Thanks for playing!");
                break;
            },
            RockPaperScissors::Invalid => {
                println!("Invalid input!");
                continue;
            },
            _ => {
                let computer_choice = computer_guess();
                println!("You chose: {}", player_choice.to_string());
                println!("Computer chose: {}", computer_choice.to_string());

                // If player wins, increment player_score
                if player_choice.beats(&computer_choice) {
                    println!("You win!");
                    player_score += 1;
                } else if computer_choice.beats(&player_choice) {
                    println!("You lose!");
                } else {
                    println!("Draw!");
                    draw_count += 1;
                }
                game_count += 1;
            }
        }

        if prompt_input("Wanna continue? (Y/n) ") == "n" {
            println!("Bye!");
            break;
        }
    }
}
