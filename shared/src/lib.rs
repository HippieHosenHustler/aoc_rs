use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Trait that all day solutions must implement
pub trait DaySolution {
    /// Solve both parts of the day's puzzle and print results
    fn solve(input_file: &str);
}

/// Trait that all year solutions must implement
pub trait YearSolution {
    /// pass the day and input file to solve the puzzle
    fn solve(day: u32, input_file: &str);
}

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    //Ok(io::BufReader::new(file).lines())

    BufRead::lines(io::BufReader::new(file))
        .map(|line| line.expect("Could not read line"))
        .collect()
}

/// Macro to automatically register day solutions
#[macro_export]
macro_rules! register_days {
    // Patern: match one or more "number => path" pairs
    ($day_var:ident, $input_var:ident, { $( $day:expr => $module:ident ),+ $(,)? }) => {
        match $day_var {
            // For each pair, generate a match arm
            $(
                $day => $module::solve($input_var),
            )+
            // Default case for unimplemented days
            _ => {
                eprintln!("Error: Day {} is not yet implemented.", $day_var);
                std::process::exit(1);
            }
        }
    };
}
