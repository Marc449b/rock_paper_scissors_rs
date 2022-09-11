use std::io::{self, Write};

pub fn prompt_input(prompt: &str) -> String {
    // Start out by printing prompt
    print!("{}", prompt);
    // flush() is used to make sure that the prompt is printed before user input
    io::stdout().flush().expect("Error: Flush failed!");

    // Finally get and return user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Read failed!");
    return input.trim().to_string();
}