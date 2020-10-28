/*
    This program gets a zipcode from a user and returns the weather for that zipcode.
    You can pass the zipcode as a command line parameter, or give it to the program when prompted.

    As this is an educational project, it is heavily commented.
*/

// This is the environment module, allows us to access the command line parameters
use std::env;

// We need std::io and std::io::Write for the IO operations.
use std::io::{self, Write};

// The start point of the program.
fn main() {
    // This stores the command line argmuents in the `args` string vector.
    let args: Vec<String> = env::args().collect();

    // Here we set the zipcode based on the if expression.
    // If there are at least 2 arguments in args, we expect the user passed in the zipcode,
    // which should be at args[1]. If so, "return" this in the if expression.
    // If they didn't pass the zipcode in, prompt them for it, and "return" in the if expression.
    let zipcode = if args.len() >= 2 {
        args[1].clone()
    } else {
        get_zipcode_from_user()
    };

    // Let the user know the zipcode we will use
    println!("The zipcode entered is {}", zipcode);

    // Print the weather based on the zipcode given
    get_weather();
}

/*
    This function handles the user input to get the zipcode from the user.

    TODO: This does not yet handle the case if the zipcode is empty / does not meet input validation
*/
fn get_zipcode_from_user() -> String {
    // Make a new mutable string named `zipcode` which will hold the user input
    let mut zipcode = String::new();

    // Prompt the user for the zipcode & flush stdout (println!() flushes but not print!())
    print!("Please enter a zipcode to use: ");
    io::stdout().flush().unwrap();

    // Read the line in on the standard input into `zipcode`.
    // `read_line()` returns a result, so unwrap it with match.
    // If there is an error, print the error.
    // Otherwise, don't do anything (zipcode already has the user input at that point).
    match io::stdin().read_line(&mut zipcode) {
        Err(error) => println!("error: {}", error),
        _ => {}
    }

    // Return the zipcode.
    zipcode
}

/*
    Get the weather from the weather api based on a valid zipcode.
    We are assuming the zipcode is valid when passing it to the api.

    You will need a valid open weather map api key:
*/
fn get_weather() {
    // Set body to the returned value from the synchronous HTTP GET call.
    let body = reqwest::blocking::get("http://api.openweathermap.org/data/2.5/weather?zip=,US&appid=")
        .unwrap()
        .text();

    // Print the weather
    println!("{:?}", body);
}
