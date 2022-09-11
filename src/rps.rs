use crate::utils::prompt_input;

use rand::prelude::*;

pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

impl RockPaperScissors {
    pub fn beats(&self, other: &RockPaperScissors) -> bool {
        return match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => true,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => true,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => true,
            _ => false,
        };
    }

    pub fn to_string(&self) -> &'static str {
        return match self {
            RockPaperScissors::Rock => "Rock",
            RockPaperScissors::Paper => "Paper",
            RockPaperScissors::Scissors => "Scissors",
            RockPaperScissors::Invalid => "Invalid",
        };
    }
}

pub fn get_input(prompt: &str) -> RockPaperScissors {
    let inp = prompt_input(prompt);

    return match inp.to_lowercase().as_str() {
        "r" | "rock" => RockPaperScissors::Rock,
        "p" | "paper" => RockPaperScissors::Paper,
        "s" | "scissors" => RockPaperScissors::Scissors,
        _ => RockPaperScissors::Invalid,
    };
}

pub fn computer_guess() -> RockPaperScissors {
    let mut rng = rand::thread_rng();

    return match rng.gen_range(0..=2) {
        0 => RockPaperScissors::Rock,
        1 => RockPaperScissors::Paper,
        _ => RockPaperScissors::Scissors,
    };
}
