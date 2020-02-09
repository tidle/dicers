mod args;
mod dat;
mod dice;

use args::Args;
use dice::roll;
use dice::DiceFace;
use dice::Roll::*;

fn main() {
    let args = Args::new();
    let mut password = String::new();
    for _ in 0..args.words {
        let result = roll(args.manual, args.long);
        password.push_str(&match result {
            Roll4(v) => format!(
                "{} ",
                short_word(v, &args.wordlist).unwrap_or_else(|e| {
                    eprintln!("Error getting word: {}", e);
                    std::process::exit(128)
                })
            ),
            Roll5(v) => format!(
                "{} ",
                long_word(v, &args.wordlist).unwrap_or_else(|e| {
                    eprintln!("Error getting word: {}", e);
                    std::process::exit(128)
                })
            ),
        });
    }

    println!("{}", password);

    // Prevent an overflow in the following code
    if args.words * if args.long { 5 } else { 4 } > 48 {
        println!("Your password is complex enough");
        return;
    }
    println!(
        "Password complexity (Number of possible passwords with given settings): {}",
        6u128.pow(args.words as u32 * if args.long { 5 } else { 4 })
    )
}

fn short_word(dice: [DiceFace; 4], wordlist: &Option<String>) -> std::io::Result<String> {
    let match_str = format!("{}{}{}{}", dice[0], dice[1], dice[2], dice[3]);
    word(&match_str, wordlist, dat::LIST4)
}

fn long_word(dice: [DiceFace; 5], wordlist: &Option<String>) -> std::io::Result<String> {
    let match_str = format!("{}{}{}{}{}", dice[0], dice[1], dice[2], dice[3], dice[4]);
    word(&match_str, wordlist, dat::LIST5)
}

fn word(match_str: &str, filename: &Option<String>, fblist: &str) -> std::io::Result<String> {
    use std::io::BufRead;
    use std::{fs, io};

    let wordlist: Box<dyn BufRead>;
    if let Some(f) = filename {
        let w = fs::File::open(f)?;
        wordlist = Box::new(io::BufReader::new(w));
    } else {
        wordlist = Box::new(io::BufReader::new(fblist.as_bytes()))
    }
    let mut result = String::new();

    for line in wordlist.lines() {
        let line = line?;
        if let Some(0) = line.find(match_str) {
            result = String::from(match line.split("\t").nth(1) {
                Some(s) => s,
                None => "[error]",
            });
            break;
        }
    }

    Ok(result)
}
