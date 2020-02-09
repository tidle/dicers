# Rsdice
**Rsdice is a diceware password generator written in rust**

## Installation
### Cargo
The recommended way to install dicers is through `cargo install`.
Assuming you have cargo installed, this will compile dicers and add it to your path.
### Executables
Executables for your platform may be available in the latest release.
Check the releases tab of the github page (https://github.com/tidle/rsdice/)
### Source
Simply execute `cargo build --release`, and the `rsdice` executable will be found at `target/release/rsdice`

## Use
You can run dicers with no arguments and it will generate a password with six words using the short wordlist.
For password customization, please refer to the help message:
```
USAGE:
    dicers [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --long       use the six dice word list instead of the four dice one
    -m, --manual     manually input dice rolls rather than using the computer
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>           manually specify a wordlist, rather than using the built in wordlists
    -w, --words <WORD COUNT>    length of password [default: 6]
```