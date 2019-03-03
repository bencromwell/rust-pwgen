mod cli;
mod pwgen;

fn main() {
    let matches = cli::build_cli().get_matches();

    let mut charsets: Vec<_> = vec![];

    if matches.is_present("character sets") {
        charsets = matches.values_of("character sets").unwrap().collect();
    } else if matches.is_present("all") {
        charsets = vec!["u", "s", "d"]
    }

    let length = matches.value_of("length").unwrap().parse::<i32>().unwrap();
    let number = matches.value_of("number").unwrap().parse::<i32>().unwrap();
    let use_safer = matches.is_present("safe");
    let bcrypt = matches.is_present("bcrypt");

    let generator = pwgen::PasswordGenerator::new(length, number, charsets, bcrypt, use_safer);
    generator.generate_all_passwords();
}
