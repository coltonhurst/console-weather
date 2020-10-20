// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b7ce05386e388cf30a5cd8b8d33297e0

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let zipcode = if args.len() >= 2 {
        args[1].clone()
    } else {
        get_zipcode_from_user()
    };

    println!("The zipcode is {}", zipcode);
}

fn get_zipcode_from_user() -> String {
    let mut zipcode = String::new();
    print!("Please enter a zip code to use: ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut zipcode) {
        Err(error) => println!("error: {}", error),
        _ => {}
    }

    zipcode
}
