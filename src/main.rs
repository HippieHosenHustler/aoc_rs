use std::env;
use std::process;

mod day01;
mod day02;
mod file_helpers;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <number> <input_file>", args[0]);
        process::exit(1);
    }

    let number: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: The first argument must be a valid unsigned integer.");
            process::exit(1);
        }
    };

    let input_file = &args[2];

    match number {
        1 => day01::solve(input_file),
        2 => day02::solve(input_file),
        // add more cases as needed
        _ => {
            eprintln!("Error: Unsupported number '{}'.", number);
            process::exit(1);
        }
    }
}
