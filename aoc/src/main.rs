use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <year> <day> <input_file>", args[0]);
        eprintln!("Example: aoc 2025 4 input.txt");
        process::exit(1);
    }

    let year: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Year must be a valid number.");
            process::exit(1);
        }
    };

    let day: u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Day must be a valid number.");
            process::exit(1);
        }
    };

    let input_file = &args[3];

    match year {
        2025 => aoc2025::solve(day, input_file),
        _ => {
            eprintln!("Error: Year {} is not yet implemented.", year);
            eprintln!("Available years: 2025");
            process::exit(1);
        }
    }
}
