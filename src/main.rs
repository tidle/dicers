mod args;
mod dice;

use args::Args;
use dice::roll;
use dice::Roll::*;

fn main() {
    let args = Args::new();
    let result = roll(args.manual, args.long);
    match result {
        Roll4(v) => println!("{}", v[0]),
        Roll5(v) => println!("{}", v[0])
    }
}

fn short_word(dice: [u8; 4]) -> std::io::Result<String> {
    use std::{fs, io};
    use std::io::BufRead;

    let matchstr = format!("{}{}{}{}", dice[0], dice[1], dice[2], dice[3]);

    let wordlist = fs::File::open("list4.txt")?;
    let wordlist = io::BufReader::new(wordlist);
    let mut result = String::new();

    for line in wordlist.lines() {
        let line = line?;
        if let Some(0) = line.find(&matchstr) {
            result = String::from( match line.split(" ").nth(1) {
                Some(s) => s,
                None => continue,
            });
            break;
        }
    }

    Ok(result)
}