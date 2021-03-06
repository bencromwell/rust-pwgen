use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    App::new("pwrust")
        .version(VERSION)
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
                .help("Always includes lowercase letters. Set u for uppercase letters, d for digits, s for symbols")
                .conflicts_with("all")
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .takes_value(false)
                .help("Include all character sets")
                .conflicts_with("character sets")
        )
        .arg(
            Arg::with_name("safe")
                .short("s")
                .long("safe")
                .takes_value(false)
                .help("Use the 'safer' symbol set")
        )
}
