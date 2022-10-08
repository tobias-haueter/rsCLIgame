use std::{io};
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
use figlet_rs::FIGfont;


fn cli_figlet_rs(){
    // https://crates.io/crates/figlet_rs
    // use figlet_rs::FIGfont;

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Hello Rust");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());

}


fn cli_guess_game(){

    // figlet_rs
    let standard_font = FIGfont::standard().unwrap();

    //print title
    let title = standard_font.convert("CLI_GG");
    assert!(title.is_some());   
    println!("{}", title.unwrap());
    
    println!("{}", "------------------------------------------------------------------------".yellow());
    println!("{}", "Guess the number! - RUST CLI GUESS GAME [rsCLI_GG])".yellow());
    println!("{}", "------------------------------------------------------------------------".yellow());
    
    // variables
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;
    let mut name = String::new();

    println!("Hello, what's your name?");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("{}", "------------------------------------------------------------------------".yellow());

    let inputname = standard_font.convert(&name);
    assert!(inputname.is_some());

    println!("Hello, ");
    println!("{}", inputname.unwrap());
    println!("nice to have you here!");

    println!("{}", "------------------------------------------------------------------------".yellow());

    println!("OK, i have a secret number.... let's start the game!");

    loop {
        count += 1;

        println!(
            "{} {} {}",
            "Please input your guess #".yellow(),
            count.to_string().red(),
            ":".yellow()
        );

        

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".blue().bold()),
                            
            Ordering::Greater => println!("{}", "Too big!".red().bold()),
            
            Ordering::Equal => {
                println!("{}", "------------------------------------------------------------------------".green());
                println!("{}", "You won!".on_green().bold());
                println!("{}", "------------------------------------------------------------------------".green());
                break;
            }
        }
    }


}

fn main() {

    cli_guess_game();
    // cli_figlet_rs();

}
