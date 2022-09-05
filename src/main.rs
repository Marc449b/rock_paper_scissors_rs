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
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => true,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => true,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => true,
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

    return match rng.gen_range(0..=2) {
        0 => RockPaperScissors::Rock,
        1 => RockPaperScissors::Paper,
        _ => RockPaperScissors::Scissors,
    };
}

struct Score {
    player: u32,
    computer: u32,
    draws: u32
}

impl Score {
    fn new() -> Score {
        Score {
            player: 0,
            computer: 0,
            draws: 0
        }
    }

    fn get_gamecount(&self) -> u32 {
        self.player + self.computer + self.draws
    }

    fn to_string(&self) -> String {
        format!("Player: {}\nComputer: {}\nDraws: {}\nGamecount: {}", self.player, self.computer, self.draws, self.get_gamecount())
    }

    fn incr_player(&mut self) {
        self.player += 1;
    }

    fn incr_comp(&mut self) {
        self.computer += 1;
    }
    
    fn incr_draw(&mut self) {
        self.draws += 1;
    }
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
        let player_choice = get_input("Rock, Paper or Scissors? q to quit: ");

        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        match player_choice {
            // Quit game
            RockPaperScissors::Quit => {
                println!("Thanks for playing!");
                break;
            },
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
