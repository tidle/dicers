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
