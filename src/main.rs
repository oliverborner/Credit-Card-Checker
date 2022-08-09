use colored::*;
use regex::Regex;
use std::io;

pub struct Issuer {
    pub pattern: Regex,
    pub name: String,
}

impl Issuer {
    fn new(regexp: &str, name: &str) -> Option<Self> {
        let pattern = Regex::new(regexp).ok()?;
        let name = String::from(name);
        Some(Issuer { pattern, name })
    }
}

pub fn make_issuer_list() -> Vec<Issuer> {
    let all = vec![
        Issuer::new(r"^3[47][0-9]{0,}$", "American Express"),
        Issuer::new(r"^(?:2131|1800|35)[0-9]{0,}$", "JCB"),
        Issuer::new(r"^3(?:0[0-59]{1}|[689])[0-9]{0,}$", "Dinersclub"),
        Issuer::new(r"^4[0-9]{0,}$", "Visa"),
        Issuer::new(
            r"^(5[1-5]|222[1-9]|22[3-9]|2[3-6]|27[01]|2720)[0-9]{0,}$",
            "Mastercard",
        ),
        Issuer::new(r"^(5[06789]|6)[0-9]{0,}$", "Maestro"),
        Issuer::new(
            r"^(6011|65|64[4-9]|62212[6-9]|6221[3-9]|622[2-8]|6229[01]|62292[0-5])[0-9]{0,}$",
            "Discover",
        ),
    ];

    all.into_iter().flatten().collect()
}

fn main() {
    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    let cc = '\u{1F4B3}';

    println!("{} Credit Card Checker", cc);
    println!("Enter CC number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // remove whitespaces and check if user input
    // contains numbers and is not empty
    input.retain(|c| !c.is_whitespace());

    if input.trim().is_empty() || !input.trim().chars().all(char::is_numeric) {
        println!("Your given input was invalid");
    } else {
        let mut digits: Vec<u32> = input.chars().flat_map(|ch| ch.to_digit(10)).collect();

        let is_valid = card_validation(&mut digits);
        detect_card_type(is_valid, &input);
    }
}

fn card_validation(digits: &mut Vec<u32>) -> bool {
    // Luhn algorithm

    // store last digit and remove from digits vec
    // then double each digit at an even index
    let valid = '\u{2705}';
    let invalid = '\u{274C}';

    let check_digit = digits.last().cloned().unwrap();
    let final_length = digits.len().saturating_sub(1);
    digits.truncate(final_length);
    digits.reverse();

    let mut processed_digits = Vec::new();

    for (index, _digit) in digits.iter().enumerate() {
        if index % 2 == 0 {
            let mut doubled_digit = digits[index] * 2;

            if doubled_digit > 9 {
                doubled_digit -= 9;
            }
            processed_digits.push(doubled_digit);
        } else {
            processed_digits.push(digits[index]);
        }
    }

    let total = check_digit + processed_digits.iter().sum::<u32>();

    let validation_result: bool = if total % 10 == 0 {
        println!("{} {}", valid, "CC is valid!".green());
        true
    } else {
        println!("{} {}", invalid, "CC is invalid!".red());
        false
    };

    validation_result
}

fn detect_card_type(is_valid: bool, input: &str) {
    // detect the cardtype with a regex pattern match
    // on the IIN card numbers

    if is_valid {
        let issuers = make_issuer_list();
        let mut matches = false;
        for i in &issuers {
            if i.pattern.is_match(input) {
                println!("{}", i.name.blue());
                matches = true;
            }
        }
        if !matches {
            println!("{}", "Type unknown".red());
        }
    }
}
