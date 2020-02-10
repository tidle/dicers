use clap::App;
use clap::Arg;

pub struct Args {
    pub short: bool,
    pub manual: bool,
    pub words: usize,
    pub inet: bool,
    pub wordlist: Option<String>,
}

impl Args {
    pub fn new() -> Args {
        let app = App::new("rsdice")
            .version("1.0")
            .about("generates a diceware password")
            .author("tidle")
            .arg(
                Arg::with_name("short")
                    .short("s")
                    .long("short")
                    .help("use the four dice word list instead of the five dice one"),
            )
            .arg(
                Arg::with_name("manual")
                    .short("m")
                    .long("manual")
                    .help("manually input dice rolls rather than using the computer"),
            )
            .arg(
                Arg::with_name("inet")
                    .short("i")
                    .long("internet")
                    .conflicts_with("manual")
                    .help("get random numbers from an internet source (random.org) rather than using the built in random number generation"),
            )
            .arg(
                Arg::with_name("FILE")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .help("manually specify a wordlist, rather than using the built in wordlists"),
            )
            .arg(
                Arg::with_name("WORD COUNT")
                    .short("w")
                    .long("words")
                    .default_value("6")
                    .takes_value(true)
                    .help("length of password"),
            )
            .get_matches();
        let short = app.is_present("short");
        let manual = app.is_present("manual");
        let words = app
            .value_of("WORD COUNT")
            .unwrap()
            .parse()
            .unwrap_or_else(|e| {
                eprintln!("Invalid number: {}", e);
                std::process::exit(60);
            });
        let wordlist = app.value_of("FILE").map(|x| x.to_string());
        let inet = app.is_present("inet");

        Args {
            short,
            manual,
            words,
            wordlist,
            inet,
        }
    }
}
