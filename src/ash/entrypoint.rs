use anyhow::{Result, Context};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Cli {
    input_string: Option<String>,
}

impl Cli {

    const AUTHOR: &'static str = "Ashley G. Otchengco";
    const ABOUT: &'static str = "A SIMPLE PROGRAM TO COUNT A STRING OF CHARACTERS.";

    // function para maka create ng bagong Cli struct
    fn parse() -> Result<Self> {
        Ok(Cli { input_string: None })
    }
}

// main function
pub fn run() -> Result<()> {
    // author and about information from the Cli struct
    println!("Author: {}", Cli::AUTHOR);
    println!("About: {}", Cli::ABOUT);

    // parse CLI arguments and retrieve the initial input string
    let mut initial_input_string = Cli::parse().context("Test CLI")?.input_string;

    // main loop
    loop {
        // tukuyin ang input_string dipende kung ang initial_input_string ay may halaga
        let input_string = if let Some(input) = initial_input_string.take() {
            input
        } else {
            // enter a string
            print!("Enter a string: ");
            io::stdout().flush()
                .context("Test flush")?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .context("Test readline from stdin")?; // 
            input.trim().to_string() // trim whitespace and convert sa string
        };

        println!("\nCharacter Count:");

        // HashMap para ma store ang character counts
        let mut char_counts = HashMap::new();

        // count each alphabetic character in the input string
        for ch in input_string.chars() {
            if ch.is_alphabetic() {
                *char_counts.entry(ch.to_ascii_lowercase()).or_insert(0) += 1; // increment count para sa character
            }
        }

        // sort the character alphabetically
        let mut sorted_counts: Vec<_> = char_counts.iter().collect();
        sorted_counts.sort_by_key(|(ch, _)| *ch);

        // print sorted character counts
        for (ch, count) in sorted_counts {
            println!("{} - {}", ch, count);
        }

        // calculate the tolta string length with and without spaces
        let slen = input_string.len();
        let slenwo = input_string.chars().filter(|&ch| ch != ' ').count();

        // display string length information
        println!("\nString count: {}", slen);
        println!("String count w/o spaces: {}", slenwo);

        // I-prompt ang user na maglagay ng 1 para magpatuloy o 2 para tumigil
        let choice: u32 = loop {
            println!("Enter 1 to continue or 2 to stop:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .context("Failed to read line from stdin")?;
            match input.trim().parse::<u32>() {
                Ok(1) => break Ok::<u32, anyhow::Error>(1), // Break with Ok(1) if input is 1
                Ok(2) => break Ok::<u32, anyhow::Error>(2), // Break with Ok(2) if input is 2
                _ => println!("Wrong input!"), // err msg para sa ibang input
            }
        }?;

        // exit loop
        if choice == 2 {
            println!("Program Stopped!");
            break;
        }
    }

    Ok(())
}
