use crate::errors::*;
use anyhow::Result;
use std::io;
use clap::{command, Arg, Command};

pub fn run() -> Result<()> {
    // Add your work here JB, go crazy!

    let match_result = command!()
    .subcommand(
        Command::new("my-program")
        .arg(
            Arg::new("first")
                .short('f')
                .long("first")
                .help("first command description")
        )
        .arg(
            Arg::new("second")
                .short('s')
                .long("second")
                .help("second command description")
        )
    ).about("about the program")
    .get_matches();

    println!("Title of Program here");
    // println!("1. First Option here");
    // println!("2. Second Option here");
    // println!("3. Third Option here");
    // println!("Your choice: ");

    // let mut choice = String::new();

    // io::stdin()
    //     .read_line(&mut choice)
    //     .expect("failed to read input");

    // let choice: u32 = match choice.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Please enter a number!");
    //         return Ok(());
    //     }
    // };

    // print!("You chose: {choice}");

    Err(HackathonError::NoSubmission(String::from("JB")).into())
}
