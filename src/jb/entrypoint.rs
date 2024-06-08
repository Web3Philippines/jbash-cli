use crate::errors::*;
use anyhow::Result;
use std::io::{self, Write as _};

#[derive(Debug)]
#[derive(Clone)]
enum CardTypes {
    Amex,
    JCB,
    Mastercard,
    Visa,
    Unknown
}
#[derive(Debug)]
#[derive(Clone)]
enum CardValidity {
    Valid,
    Invalid
}
#[derive(Clone)]
struct ValidationResult {
    ccn: String,
    card_type: CardTypes,
    validity: CardValidity,
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn wait_keypress() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

pub fn run() -> Result<()> {
    let mut valid_choice = true;
    let mut result_collection: Vec<ValidationResult> = Vec::new();

    while valid_choice {
        clear_screen();
        println!("CCN Validation Checker");
        println!("Please select an option: ");
        println!("1. CCN Checker");
        println!("2. History");
        println!("3. Exit");
        println!("Your choice: ");

        let choice = get_user_input();

        match choice {
            1 => result_collection = ccn_checker(&mut result_collection),
            2 => history(&result_collection),
            3 => {
                println!("\nProgram ended.");
                wait_keypress();
                valid_choice = false
            },
            _ => {
                println!("\n\nInvalid choice, enter any key to try again.\n");
                wait_keypress();
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

fn ccn_checker(result_collection: &mut Vec<ValidationResult>) -> Vec<ValidationResult> {
    clear_screen();
    println!("CCN Checker");
    println!("(Enter a 13 to 19 digit CCN)");
    println!("Enter CCN: ");

    let ccn: String = get_user_input().to_string();
    let result: ValidationResult = validate_ccn(ccn);
    println!("\n\n");
    println!("CCN: {}", result.ccn);
    println!("Card Type: {:?}", result.card_type);
    println!("Validity: {:?}", result.validity);
    println!("\nEnter any key to continue ...");
    wait_keypress();
    result_collection.push(result);
    result_collection.clone()
}

fn history(result_collection: &Vec<ValidationResult>) {
    clear_screen();
    println!("Validation History:");
    for (index, result) in result_collection.iter().enumerate() {
        println!("{}. CCN: {}", index + 1, result.ccn);
        println!("   Card Type: {:?}", result.card_type);
        println!("   Validity: {:?}", result.validity);
    }
    println!("\nEnter any key to continue ...");
    wait_keypress();
}

fn validate_ccn(ccn: String) -> ValidationResult {
    let valid_length: bool = validate_ccn_length(&ccn);
    let validity: CardValidity = verify_validity(&ccn);
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
                    validity,
                };
            }
        }
    } else {
        println!("\nInvalid Input! Please enter valid digits only.");
    }

    return ValidationResult {
        ccn: ccn.clone(),
        card_type: CardTypes::Unknown,
        validity: CardValidity::Invalid,
    };
}

fn verify_validity(ccn: &String) -> CardValidity {
    let mut sum_of_even: u32 = 0;
    let mut sum_of_odd: u32 = 0;

    for num in ccn.chars().rev().step_by(2) {
        if let Some(digit) = num.to_digit(10) {
            sum_of_odd += digit;
        }
    }

    for num in ccn.chars().rev().skip(1).step_by(2) {
        if let Some(digit) = num.to_digit(10) {
            let double_digit = digit * 2;
            if double_digit >= 10 {
                let even_temp1 = double_digit.to_string();
                let first_digit = even_temp1.chars().nth(0).unwrap().to_digit(10).unwrap();
                let second_digit = even_temp1.chars().nth(1).unwrap().to_digit(10).unwrap();
                sum_of_even += first_digit + second_digit;
            } else {
                sum_of_even += double_digit;
            }
        }
    }

    let sum = sum_of_odd + sum_of_even;

    if sum % 10 == 0 {
        return CardValidity::Valid;
    } else {
        return CardValidity::Invalid;
    }
}

fn validate_ccn_length(ccn: &String) -> bool {
    ccn.len() > 12 && ccn.len() < 20
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
    } else if type_num >= 40 && type_num <= 49 {
        return CardTypes::Visa;
    } else {
        return CardTypes::Unknown;
    }
}