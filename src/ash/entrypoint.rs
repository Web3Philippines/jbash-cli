use crate::errors::*;
use anyhow::Result;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    
    // Define the Cli struct with AUTHOR, ABOUT
    struct Cli {
        input_string: Option<String>,
    }

    impl Cli {
        const AUTHOR: &'static str = "Ashley G. Otchengco";
        const ABOUT: &'static str = "A SIMPLE PROGRAM TO COUNT A STRING OF CHARACTERS.";

        fn parse() -> Self {
            Cli { input_string: None }
        }
    }

    println!("Author: {}", Cli::AUTHOR);
    println!("About: {}", Cli::ABOUT);

    loop {
        let cli = Cli::parse();

        let input_string = match cli.input_string {
            Some(string) => string,
            None => {
                print!("Enter a string: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            }
        };

        println!("\nCharacter Count:");

        let slen = input_string.len();

        for (_x, ch) in input_string.chars().enumerate() {
            let mut ctr = 0;
            print!("{}-", ch);
            if ch.is_alphabetic() {
                ctr += 1;
            }
            println!("{}", ctr);
        }

        let slenwo = input_string.chars().filter(|&ch| ch != ' ').count();

        println!("\nString count: {}", slen);
        println!("String count w/o spaces: {}", slenwo);

        // continue or stop
        println!("Enter 1 to continue or 2 to stop:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        if choice == 2 {
            println!("Program Stopped!");
            break;
        }
    }

    Err(HackathonError::NoSubmission(String::from("Ash")).into())
}