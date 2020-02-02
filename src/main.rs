mod args;
mod dice;

use args::Args;
use dice::roll;
use dice::Roll::*;

fn main() {
    let args = Args::new();
    let result = roll(false, args.long);
    match result {
        Roll4(v) => println!("{}", v[0]),
        Roll6(v) => println!("{}", v[0])
    }
}
