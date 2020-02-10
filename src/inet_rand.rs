use super::args::Args;
use super::dice::{DiceFace, Roll};

pub fn roll(config: &Args) -> Roll {
    let count = if config.short { 5 } else { 4 };

    eprintln!("starting request...");
    let res: Vec<DiceFace> = ureq::get("https://random.org/integers/")
        .query("num", &format!("{}", count))
        .query("min", "1")
        .query("max", "6")
        .query("col", "1")
        .query("base", "10")
        .query("format", "plain")
        .query("rnd", "new")
        .call()
        .into_string()
        .unwrap_or_else(|e| {
            eprintln!("encountered error making request to random.org: {}", e);
            std::process::exit(200)
        })
        .split("\n")
        .take(count)
        .map(|x| {
            x.parse::<u8>()
                .map(|y| DiceFace::new(y))
                .unwrap_or_else(|e| {
                    eprintln!("encountered error processing request to random.org: {}", e);
                    std::process::exit(201)
                })
                .unwrap_or_else(|| {
                    eprintln!("got invalid number from random.org");
                    std::process::exit(201)
                })
        })
        .collect();
    
    if res.len() < count {
        eprintln!("list of numbers from random.org too short! expected {}, got {}", count, res.len());
        std::process::exit(202)
    }
    if count == 4 {
        Roll::Roll4([res[0], res[1], res[2], res[3]])
    } else {
        Roll::Roll5([res[0], res[1], res[2], res[3], res[4]])        
    }
}
