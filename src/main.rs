extern crate bcrypt;
extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let matches = App::new("pwrust")
        .version("1.0.0")
        .about("Generates passwords")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .default_value("20")
                .help("The length of the password to generate")
                .index(1)
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .default_value("1")
                .help("The number of passwords to generate")
        )
        .arg(
            Arg::with_name("bcrypt")
                .short("b")
                .long("bcrypt")
                .takes_value(false)
                .help("Output the bcrypt hash for the password generated")
                .conflicts_with("number")
        )
        .arg(
            Arg::with_name("character sets")
                .short("c")
                .long("chars")
                .multiple(true)
                .takes_value(true)
                .use_delimiter(false)
                .possible_values(&["u", "d", "s"])
                .help("Always includes lowercase letters. U for uppercase letters, d for digits, s for symbols")
        )
        .get_matches();

    let length = matches.value_of("length").unwrap().parse::<i32>().unwrap();
    let number = matches.value_of("number").unwrap().parse::<i32>().unwrap();

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let symbols: Vec<_> = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();
    let letters: Vec<_> = alphabet.chars().collect();
    let letters_lower: Vec<_> = alphabet.to_lowercase().chars().collect();
    let digits: Vec<_> = "0123456789".chars().collect();

    let mut num_pws_generated: i32 = 0;

    let mut charsets: Vec<_> = vec![];

    if matches.is_present("character sets") {
        charsets = matches.values_of("character sets").unwrap().collect();
    }

    while num_pws_generated < number {
        let mut pw = String::new();

        while (pw.len() as i32) < length {
            let mut source = letters_lower.clone();
            // play with the range and the options matched to adjust the relative weightings of the
            // different character types
            let x = thread_rng().gen_range(1, 15);

            match x {
                1 | 2 | 3 | 4 => {
                    if charsets.contains(&"u") {
                        source = letters.clone();
                    }
                }
                5 | 6 => {
                    if charsets.contains(&"s") {
                        source = symbols.clone();
                    }
                }
                7 | 8 | 9 | 10 => {
                    if charsets.contains(&"d") {
                        source = digits.clone();
                    }
                }
                _ => {
                    // do nothing, covered by the default above
                }
            }

            let chosen_char: char = thread_rng().choose(&source).cloned().unwrap();
            pw.push(chosen_char);
        }

        num_pws_generated += 1;

        if matches.is_present("bcrypt") {
            let hashed_pw = hash(pw.as_str(), DEFAULT_COST).unwrap();
            println!("pass: {}\nhash: {}", pw, hashed_pw);
        } else {
            println!("{}", pw);
        }
    }
}
