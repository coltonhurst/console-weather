use std::io::{self, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut zipcode = String::new();

    if args.len() >= 2 {
        //zipcode = args[1];
    } else {
        print!("Please enter a zip code to use: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut zipcode) {
            Ok(_) => {}
            Err(error) => println!("error: {}", error),
        }
    }

    println!("The zipcode is {}", zipcode);
}
