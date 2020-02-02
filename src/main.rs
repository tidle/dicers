mod args;
mod dice;

use args::Args;
use dice::roll;
use dice::Roll::*;

fn main() {
    let args = Args::new();
    let result = roll(args.manual, args.long);
    match result {
        Roll4(v) => println!(
            "{}",
            short_word(v).unwrap_or_else(|e| {
                eprintln!("Error getting word: {}", e);
                std::process::exit(128)
            })
        ),
        Roll5(v) => println!(
            "{}",
            long_word(v).unwrap_or_else(|e| {
                eprintln!("Error getting word: {}", e);
                std::process::exit(128)
            })
        ),
    }
}

fn short_word(dice: [u8; 4]) -> std::io::Result<String> {
    let match_str = format!("{}{}{}{}", dice[0], dice[1], dice[2], dice[3]);
    word(&match_str, "list4.txt")
}

fn long_word(dice: [u8; 5]) -> std::io::Result<String> {
    let match_str = format!("{}{}{}{}{}", dice[0], dice[1], dice[2], dice[3], dice[4]);
    word(&match_str, "list5.txt")
}

fn word(match_str: &str, filename: &str) -> std::io::Result<String> {
    use std::io::BufRead;
    use std::{fs, io};

    let wordlist = fs::File::open(filename)?;
    let wordlist = io::BufReader::new(wordlist);
    let mut result = String::new();

    for line in wordlist.lines() {
        let line = line?;
        if let Some(0) = line.find(match_str) {
            result = String::from(match line.split("\t").nth(1) {
                Some(s) => s,
                None => "",
            });
            break;
        }
    }

    Ok(result)
}
