
mod core;

use core::get_repos;
use std::{env, fs::File, io::{BufRead, BufReader}};
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <github_access_token> <orgniaztion_list>", args[0]);
        ()
    }
    
    let token = &args[1];
    let file_path = &args[2];
    let file = File::open(file_path).expect("Couldn't open the file");
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        match line {
            Ok(_) => { 
                if let Err(err) = get_repos(&line.unwrap().as_str(), token).await {
                    eprintln!("Error fetching repos: {}", err);
                }
            },
            Err(_) => eprintln!("Error parsing"),
        }
    }   
    Ok(())
}
