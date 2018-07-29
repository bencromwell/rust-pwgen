extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::{thread_rng, Rng};

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
        .get_matches();

    let length = matches.value_of("length").unwrap().parse::<i32>().unwrap();
    let number = matches.value_of("number").unwrap().parse::<i32>().unwrap();

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let symbols: Vec<_> = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();
    let letters: Vec<_> = alphabet.chars().collect();
    let letters_lower: Vec<_> = alphabet.to_lowercase().chars().collect();
    let digits: Vec<_> = "0123456789".chars().collect();

    let mut num_pws_generated: i32 = 0;

    while num_pws_generated < number {
        let mut pw = String::new();

        while (pw.len() as i32) < length {
            let source;
            // play with the range and the options matched to adjust the relative weightings of the
            // different character types
            let x = thread_rng().gen_range(1, 15);

            match x {
                1 | 2 | 3 | 4 => {
                    source = letters.clone();
                }
                5 | 6 => {
                    source = symbols.clone();
                }
                7 | 8 | 9 | 10 => {
                    source = digits.clone();
                }
                _ => {
                    source = letters_lower.clone();
                }
            }

            let chosen_char: char = thread_rng().choose(&source).cloned().unwrap();
            pw.push(chosen_char);
        }

        num_pws_generated += 1;

        println!("{}", pw);
    }
}
