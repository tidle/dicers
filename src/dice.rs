use super::args::Args;
pub use dice_face::DiceFace;
use rand::prelude::*;
use std::io::BufRead;

pub enum Roll {
    Roll4([DiceFace; 4]),
    Roll5([DiceFace; 5]),
}

pub fn roll(config: &Args) -> Roll {
    if config.inet {
        return super::inet_rand::roll(&config);
    }

    let long = config.long;
    let ask = config.manual;

    let lim = if long { 5 } else { 4 };
    let mut rolls = Vec::with_capacity(lim);
    for _ in 0..lim {
        if ask {
            print!("Input a dice roll from 1 to 6: ");
            // Flush stdout so that the prompt appears
            use std::io::Write;
            std::io::stdout()
                .lock()
                .flush()
                .ok()
                .expect("Could not flush stdout");

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
            // Ensure value is in the proper bounds
            let value = (value - 1) % 6 + 1;
            let value = match DiceFace::new(value) {
                Some(v) => v,
                None => {
                    eprintln!("please enter a valid dice roll");
                    continue;
                }
            };
            rolls.push(value);
        } else {
            // Fix this. Not secure
            rolls.push(DiceFace::new(thread_rng().gen_range(1, 7)).unwrap());
        }
    }

    if long {
        Roll::Roll5([rolls[0], rolls[1], rolls[2], rolls[3], rolls[4]])
    } else {
        Roll::Roll4([rolls[0], rolls[1], rolls[2], rolls[3]])
    }
}

mod dice_face {
    /// Value that is guaranteed to be a valid dice roll
    #[derive(Copy, Clone)]
    pub struct DiceFace {
        val: u8,
    }

    impl DiceFace {
        /// Creates a new DiceFace
        pub fn new(val: u8) -> Option<DiceFace> {
            if val < 1 || val > 6 {
                None
            } else {
                Some(DiceFace { val })
            }
        }

        /// Gets the value contained within the DiceFace
        pub fn val(&self) -> u8 {
            self.val
        }
    }

    // Only works in rust 1.40 or later
    impl From<DiceFace> for u8 {
        fn from(df: DiceFace) -> u8 {
            df.val()
        }
    }

    impl std::fmt::Display for DiceFace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }
}
