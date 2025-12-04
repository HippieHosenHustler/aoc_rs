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
        2025 => solve_2025(day, input_file),
        _ => {
            eprintln!("Error: Year {} is not yet implemented.", year);
            eprintln!("Available years: 2025");
            process::exit(1);
        }
    }
}

fn solve_2025(day: u32, input_file: &str) {
    match day {
        1 => aoc2025::day01::solve(input_file),
        2 => aoc2025::day02::solve(input_file),
        3 => aoc2025::day03::solve(input_file),
        4 => aoc2025::day04::solve(input_file),
        _ => {
            eprintln!("Error: Day {} for year 2025 is not yet implemented.", day);
            process::exit(1);
        }
    }
}
