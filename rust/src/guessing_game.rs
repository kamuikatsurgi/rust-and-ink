use std::io;
use colored::*;
fn main() {
    loop{
        println!("{}", "Guessing the number!".blue());
        println!("{}", "Type the number: ".blue());
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to capture the number");
        println!("You have guessed the number: {}", guess);
        
        let guess: u64 = guess.trim().parse().expect("Failed to convert to integer");
        let answer: u64 = 96;
        
        if guess == answer {
            println!("{}", "You guessed the number correctly!".green());
            break;
        } else {
            println!("{}", "Sorry! Your answer did not match".red());
        }
    }

}