pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn solve(day: u32, input: &str) {
    shared::register_days!(day, input, {
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
    });
}
