use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header;
use std::{env, fs::OpenOptions, io::Write, process};

fn main() {
    let day: u8 = match aoc::parse_args() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Need to specify a day (as integer). example: `cargo download 7`");
            process::exit(1);
        }
    };
    dotenv().ok();

    let day_padded = format!("{:02}", day);
    let token = env::var("TOKEN").expect("$TOKEN is not set");
    let year = env::var("YEAR")
        .expect("$YEAR is not set")
        .parse::<u32>()
        .expect("$YEAR must be a number");

    let mut headers = header::HeaderMap::new();
    let mut session_header = header::HeaderValue::from_str(format!("session={}", token).as_str())
        .expect("Error building cookie header");
    session_header.set_sensitive(true);
    headers.insert(header::COOKIE, session_header);

    let client = Client::builder().default_headers(headers).build().unwrap();
    let res = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .send()
        .unwrap()
        .text()
        .unwrap();

    let input_path = format!("src/inputs/{}.txt", day_padded);
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&input_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {}", e);
            process::exit(1);
        }
    };

    match file.write_all(res.as_bytes()) {
        Ok(_) => {
            println!("Downloaded input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {}", e);
            process::exit(1);
        }
    }
}
