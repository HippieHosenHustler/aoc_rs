use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Trait that all day solutions must implement
pub trait DaySolution {
    /// Solve both parts of the day's puzzle and print results
    fn solve(input_file: &str);
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
