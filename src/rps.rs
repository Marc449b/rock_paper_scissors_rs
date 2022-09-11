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
