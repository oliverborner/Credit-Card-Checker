use colored::*;
use regex::Regex;
use std::io;

fn main() {
    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    // emoji
    let cc = '\u{1F4B3}';
    let valid = '\u{2705}';
    let invalid = '\u{274C}';

    println!("{} Credit Card Checker", cc);
    println!("Enter CC number:");

    // handling input
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // convert input string into digits and store it in a vec
    let mut digits: Vec<u32> = input.chars().flat_map(|ch| ch.to_digit(10)).collect();

    // ************** Luhn Algorithm **************

    // get last digit
    let check_digit = digits.last().cloned().unwrap();

    // remove last digit from vec
    digits.retain(|&x| x != check_digit);

    // reverse digits
    digits.reverse();

    // process digits
    let mut processed_digits = Vec::new();

    for index in 0..digits.len() {
        if index % 2 == 0 {
            let mut doubled_digit = digits[index] * 2;

            if doubled_digit > 9 {
                doubled_digit = doubled_digit - 9;
            }
            processed_digits.push(doubled_digit);
        } else {
            processed_digits.push(digits[index]);
        }
    }

    // get sum
    let total = return_sum(check_digit, processed_digits.iter().sum());

    // validation
    let validation_result: bool;

    if total % 10 == 0 {
        println!("{} {}", valid, "CC is valid!".green());
        validation_result = true;
    } else {
        println!("{} {}", invalid, "CC is invalid!".red());
        validation_result = false;
    }

    // detect cardtype
    if validation_result == true {
        let amex = Regex::new(r"^3[47][0-9]{0,}$").unwrap();
        let jcb = Regex::new(r"^(?:2131|1800|35)[0-9]{0,}$").unwrap();
        let dinersclub = Regex::new(r"^3(?:0[0-59]{1}|[689])[0-9]{0,}$").unwrap();
        let visa = Regex::new(r"^4[0-9]{0,}$").unwrap();
        let mastercard =
            Regex::new(r"^(5[1-5]|222[1-9]|22[3-9]|2[3-6]|27[01]|2720)[0-9]{0,}$").unwrap();
        let maestro = Regex::new(r"^(5[06789]|6)[0-9]{0,}$").unwrap();
        let discover = Regex::new(
            r"^(6011|65|64[4-9]|62212[6-9]|6221[3-9]|622[2-8]|6229[01]|62292[0-5])[0-9]{0,}$",
        )
        .unwrap();

        let formated_input = strip_trailing_newline(&input);

        if amex.is_match(formated_input) {
            println!("{}", "American Express".blue());
        } else if jcb.is_match(formated_input) {
            println!("{}", "Japan Credit Bureau".blue());
        } else if dinersclub.is_match(formated_input) {
            println!("{}", "Diners Club".blue());
        } else if visa.is_match(formated_input) {
            println!("{}", "Visa".blue());
        } else if mastercard.is_match(formated_input) {
            println!("{}", "Mastercard".blue());
        } else if maestro.is_match(formated_input) {
            println!("{}", "Maestro".blue());
        } else if discover.is_match(formated_input) {
            println!("{}", "Discover".blue());
        } else {
            println!("{}", "Type unknown".red());
        }
    }
}

// helpers
fn return_sum(i: u32, j: u32) -> u32 {
    i + j
}

// remove \n from inputstring
fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}
