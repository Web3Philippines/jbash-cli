use crate::errors::*;
use anyhow::Result;
use std::{io};

#[derive(Debug)]
enum CardTypes {
    Amex,
    JCB,
    Mastercard,
    Visa,
    Unknown
}

struct ValidationResult {
    ccn: String,
    card_type: CardTypes,
    validity: bool,
}


pub fn run() -> Result<()> {
    // Add your work here JB, go crazy!

    let mut valid_choice: bool = true;

    while valid_choice {
        println!("CCN Validation Checker");
        println!("Please select an option: ");
        println!("1. CCN Checker");
        println!("2. History");
        println!("3. Exit");
        println!("Your choice: ");

        let choice = get_user_input();

        match choice {
            1 => valid_choice = ccn_checker(),
            2 => history(),
            3 => valid_choice = false,
            _ => {
                println!("\n\nInvalid choice, please enter again.\n");
            }
        }
    }

    Err(HackathonError::NoSubmission(String::from("JB")).into())
}

fn get_user_input() -> usize {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return 0;
        }
    };
    input
}

fn ccn_checker() -> bool {
    println!("CCN Checker");
    println!("(Enter a 13 to 19 digit CCN)");
    println!("Enter CCN: ");

    let ccn: String = get_user_input().to_string();

    // println!("CCN: {ccn}");
    let result: ValidationResult = validate_ccn(ccn);
    print!("CCN: {}", result.ccn);
    print!("Card Type: {:?}", result.card_type);
    print!("Validity: {}", result.validity);

    return true;
}

fn history() {
    print!("History");
}

fn validate_ccn(ccn: String) -> ValidationResult {
    let valid_length: bool = validate_ccn_length(&ccn);

    let card_type: CardTypes = get_card_type(&ccn);

    if valid_length {
        for character in ccn.chars() {
            if !character.is_digit(10) {
                println!("\nInvalid Input! Please enter valid digits only.");
                break;
            } else {
                return ValidationResult {
                    ccn,
                    card_type,
                    validity: true,
                };
            }
        }
    } else {
        println!("\nInvalid Input! Please enter valid digits only.");
    }

    return ValidationResult {
        ccn: ccn.clone(),
        card_type: CardTypes::Unknown,
        validity: false,
    };
}

fn validate_ccn_length(ccn: &String) -> bool {
    let mut result: bool = false;
    if ccn.len() > 12 && ccn.len() < 20 {
        result = true;
    };
    result
}

fn get_card_type(ccn: &String) -> CardTypes {
    let type_num_str = &ccn[0..2];
    let type_num: i32 = type_num_str.parse().expect("Failed to parse integer");

    if type_num == 34 || type_num == 37 {
        return CardTypes::Amex;
    } else if type_num == 35 {
        return  CardTypes::JCB;
    } else if type_num < 56 && type_num > 50 {
        return CardTypes::Mastercard;
    } else {
        let type_num_str = &ccn[..0];
        let type_num: i32 = type_num_str.parse().expect("Failed to parse integer");
        if type_num == 4 {
            return CardTypes::Visa;
        } else {
            return CardTypes::Unknown;
        }
       
    }
}