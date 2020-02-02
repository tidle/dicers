use clap::App;
use clap::Arg;

pub struct Args {
    pub long: bool,
    pub manual: bool,
}

impl Args {
    pub fn new() -> Args {
        let app = App::new("dicers")
            .version("0.1")
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
            .get_matches();
        let long = app.is_present("long");
        let manual = app.is_present("manual");
        Args {
            long, manual
        }
    }
}
