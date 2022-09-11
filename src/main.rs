mod rps;
mod score;
mod utils;

use rps::{
    RockPaperScissors,
    get_input,
    computer_guess,
};
use score::Score;
use utils::prompt_input;

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
