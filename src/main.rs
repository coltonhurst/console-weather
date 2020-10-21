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

#[tokio::main]
async fn get_weather(zipcode: String) -> Result<(), reqwest::Error> {
    // example get
    let res = reqwest::get("https://example.com").await?;

    println!("The zipcode is {}", zipcode);
    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}
