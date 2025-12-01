use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Rotation {
    dir: Direction,
    value: i32,
}

enum Direction {
    L,
    R,
}

pub fn solve(input_file: &str) {
    let mut position: i32 = 50;
    let mut zero_counter = 0;

    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let rotation = parse_rotation(&line);
            position = get_next_position(position, &rotation);
            if position == 0 {
                zero_counter += 1;
            }
        }
    }

    println!("Number of zeroes: {}", zero_counter);
}

fn parse_rotation(s: &str) -> Rotation {
    let (dir_char, value_str) = s.split_at(1);
    let dir = match dir_char {
        "L" => Direction::L,
        "R" => Direction::R,
        _ => panic!("Invalid direction"),
    };
    let value: i32 = value_str.parse::<i32>().expect("Invalid number");
    Rotation { dir, value }
}

fn get_next_position(current_position: i32, rotation: &Rotation) -> i32 {
    let mut result: i32 = current_position;
    match rotation.dir {
        Direction::L => {
            result -= rotation.value;
        }
        Direction::R => {
            result += rotation.value;
        }
    };

    while result >= 100 {
        result -= 100;
    }
    while result < 0 {
        result += 100;
    }

    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
