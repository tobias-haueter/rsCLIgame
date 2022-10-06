use std::{io};
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("{}", "------------------------------------------------------------------------".yellow());
    println!("{}", "Guess the number! - RUST CLI GAME :-)".yellow());
    println!("{}", "------------------------------------------------------------------------".yellow());
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count = 0;

    loop {
        count += 1;

        println!("Please input your guess #{count}:");

        

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".magenta().bold()),
                            
            Ordering::Greater => println!("{}", "Too big!".red().bold()),
            
            Ordering::Equal => {
                println!("{}", "------------------------------------------------------------------------".green());
                println!("{}", "You win!".on_green().bold());
                println!("{}", "------------------------------------------------------------------------".green());
                break;
            }
        }
    }

    

}