use rand::prelude::*;
use std::io::BufRead;

pub enum Roll {
    Roll4([u8; 4]),
    Roll6([u8; 6]),
}

pub fn roll(ask: bool, long: bool) -> Roll {
    let lim = if long { 6 } else { 4 };
    let mut rolls = vec![0u8; lim];
    for _ in 0..lim {
        if ask {
            print!("Input a dice roll from 1 to 6: ");
            let value = std::io::stdin()
                .lock()
                .lines() // Get the lines
                .next() // The next line
                .map(|v| {
                    // Unwrap the value within the Option returned by next(), or return an ""
                    v.unwrap_or(String::new())
                })
                .unwrap_or(String::new()) // Unwrap the outer Option, or return a ""
                .parse::<u8>() // Parse it into a numer, or
                .unwrap_or_else(|e| {
                    // Unwrap it and print the error
                    eprintln!("Error reading input: {}", e);
                    std::process::exit(127)
                });
            rolls.push(value);
        } else {
            // Fix this. Not secure and favors 1-4 over 5-6
            rolls.push(random::<u8>() % 6 + 1);
        }
    }

    if long {
        Roll::Roll6([rolls[0], rolls[1], rolls[2], rolls[3], rolls[4], rolls[5]])
    } else {
        Roll::Roll4([rolls[0], rolls[1], rolls[2], rolls[3]])
    }
}
