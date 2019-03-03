use bcrypt::{hash, DEFAULT_COST};
use rand::{thread_rng, Rng};

const WEIGHT_RANGE_MIN: i32 = 1;
const WEIGHT_RANGE_MAX: i32 = 15;

pub struct PasswordGenerator<'a> {
    // properties
    symbols: Vec<char>,
    safer_symbols: Vec<char>,
    letters: Vec<char>,
    letters_lower: Vec<char>,
    digits: Vec<char>,
    // options
    length: i32,
    number: i32,
    use_safer: bool,
    bcrypt: bool,
    charsets: Vec<&'a str>,
}

impl<'a> Default for PasswordGenerator<'a> {
    fn default() -> PasswordGenerator<'a> {
        let alphabet: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        PasswordGenerator {
            symbols: "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect(),
            // some symbols break the selection when you double click in the terminal to copy them
            // additionally some symbols can break some bad implementations in certain websites
            safer_symbols: "#%&+-=?@_~".chars().collect(),
            letters: alphabet.chars().collect(),
            letters_lower: alphabet.to_lowercase().chars().collect(),
            digits: "0123456789".chars().collect(),
            // options
            length: 0,
            number: 0,
            use_safer: false,
            bcrypt: false,
            charsets: Vec::new(),
        }
    }
}

impl<'a> PasswordGenerator<'a> {
    pub fn new(length: i32, number: i32, charsets: Vec<&'a str>, bcrypt: bool, use_safer: bool) -> PasswordGenerator<'a> {
        PasswordGenerator {
            length,
            number,
            charsets,
            use_safer,
            bcrypt,
            ..Default::default()
        }
    }

    pub fn generate_all_passwords(&self) {
        let mut num_pws_generated: i32 = 0;

        while num_pws_generated < self.number {
            let pw = self.generate_password();
            num_pws_generated += 1;

            if self.bcrypt {
                let hashed_pw = hash(pw.as_str(), DEFAULT_COST).unwrap();
                println!("pass: {}\nhash: {}", pw, hashed_pw);
            } else {
                println!("{}", pw);
            }
        }
    }

    pub fn generate_password(&self) -> String {
        let mut pw = String::new();

        while (pw.len() as i32) < self.length {
            pw.push(self.pick_char());
        }

        pw
    }

    fn pick_char(&self) -> char {
        let mut source = &self.letters_lower;
        let x = thread_rng().gen_range(WEIGHT_RANGE_MIN, WEIGHT_RANGE_MAX);

        // play with the range and the options matched to adjust the relative weightings of the
        // different character types
        match x {
            1...5 => {
                if self.charsets.contains(&"u") {
                    source = &self.letters;
                }
            }
            6...7 => {
                if self.charsets.contains(&"s") {
                    if self.use_safer {
                        source = &self.safer_symbols;
                    } else {
                        source = &self.symbols;
                    }
                }
            }
            8...12 => {
                if self.charsets.contains(&"d") {
                    source = &self.digits;
                }
            }
            _ => {
                // do nothing, covered by the default above
            }
        }

        thread_rng().choose(source).cloned().unwrap()
    }
}
