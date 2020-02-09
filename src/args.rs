use clap::App;
use clap::Arg;

pub struct Args {
    pub long: bool,
    pub manual: bool,
    pub words: usize,
    pub inet: bool,
    pub wordlist: Option<String>,
}

impl Args {
    pub fn new() -> Args {
        let app = App::new("dicers")
            .version("0.3")
            .about("generates a diceware password")
            .author("tidle")
            .arg(
                Arg::with_name("long")
                    .short("l")
                    .long("long")
                    .help("use the six dice word list instead of the four dice one"),
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
        let long = app.is_present("long");
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
            long,
            manual,
            words,
            wordlist,
            inet,
        }
    }
}
