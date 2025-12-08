mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn solve(day: u32, input: &str) {
    shared::register_days!(day, input, {
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
        5 => day05,
        6 => day06,
    });
}
