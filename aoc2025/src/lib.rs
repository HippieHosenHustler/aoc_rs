use shared::{DaySolution, YearSolution};
use std::process;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub struct Year2025;

impl YearSolution for Year2025 {
    fn solve(day: u32, input_file: &str) {
        match day {
            1 => day01::Day01::solve(input_file),
            2 => day02::Day02::solve(input_file),
            3 => day03::Day03::solve(input_file),
            4 => day04::Day04::solve(input_file),
            _ => {
                eprintln!("Error: Day {} for year 2025 is not yet implemented.", day);
                process::exit(1);
            }
        }
    }
}
