use rand::{thread_rng, Rng};
use bcrypt::{hash, DEFAULT_COST};

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    let length = matches.value_of("length").unwrap().parse::<i32>().unwrap();
    let number = matches.value_of("number").unwrap().parse::<i32>().unwrap();

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let symbols: Vec<_> = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();
    // some symbols break the selection when you double click in the terminal to copy them
    // additionally some symbols can break some bad implementations in certain websites
    let safer_symbols: Vec<_> = "#%&+-=?@_~".chars().collect();
    let letters: Vec<_> = alphabet.chars().collect();
    let letters_lower: Vec<_> = alphabet.to_lowercase().chars().collect();
    let digits: Vec<_> = "0123456789".chars().collect();

    let mut num_pws_generated: i32 = 0;

    let mut charsets: Vec<_> = vec![];

    if matches.is_present("character sets") {
        charsets = matches.values_of("character sets").unwrap().collect();
    } else if matches.is_present("all") {
        charsets = vec!["u", "s", "d"]
    }

    while num_pws_generated < number {
        let mut pw = String::new();

        while (pw.len() as i32) < length {
            let mut source = &letters_lower;
            // play with the range and the options matched to adjust the relative weightings of the
            // different character types
            let x = thread_rng().gen_range(1, 15);

            match x {
                1 ... 5 => {
                    if charsets.contains(&"u") {
                        source = &letters;
                    }
                }
                6 ... 7 => {
                    if charsets.contains(&"s") {
                        if matches.is_present("safe") {
                            source = &safer_symbols;
                        } else {
                            source = &symbols;
                        }
                    }
                }
                8 ... 12 => {
                    if charsets.contains(&"d") {
                        source = &digits;
                    }
                }
                _ => {
                    // do nothing, covered by the default above
                }
            }

            let chosen_char: char = thread_rng().choose(source).cloned().unwrap();
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
