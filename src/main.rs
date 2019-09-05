use std::env;
use std::io;
use std::io::BufRead; // Needed to use `lines()` on the stdin handle.

fn main() {
    let argument = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: nth LINE_NUMBER");
            std::process::exit(1);
        }
    };

    // TODO: Implement help argument as `-h` or `--help` or `help`.

    let line_number: u32 = match u32::from_str_radix(&argument, 10) {
        Ok(number) => number,
        Err(_) => {
            eprintln!("nth: Invalid line number");
            std::process::exit(1);
        }
    };

    let stdin = io::stdin();
    let handle = stdin.lock();

    let line = match handle.lines().nth((line_number - 1) as usize) {
        Some(l) => l.unwrap(),
        None => {
            eprintln!("nth: Invalid line number");
            std::process::exit(1)
        }
    };

    println!("{}", line);
}
