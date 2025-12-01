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
    let mut zero_passes = 0;

    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let rotation = parse_rotation(&line);
            let result = get_next_position(position, &rotation);
            position = result.0;
            zero_passes += result.1;
            if position == 0 {
                zero_counter += 1;
            }
        }
    }

    println!("Number of zeroes (part 1 solution): {}", zero_counter);
    println!("Zero passes (part 2 solution): {}", zero_passes);
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

fn get_next_position(current_position: i32, rotation: &Rotation) -> (i32, i32) {
    let p = current_position;
    let d = rotation.value;

    let zero_passes = match rotation.dir {
        Direction::L => {
            // Moving left: count multiples of 100 in range [p-d, p)
            // Formula: floor((p-1)/100) - floor((p-d-1)/100)
            (p - 1).div_euclid(100) - (p - d - 1).div_euclid(100)
        }
        Direction::R => {
            // Moving right: count multiples of 100 in range (p, p+d]
            // Formula: floor((p+d)/100) - floor(p/100)
            (p + d).div_euclid(100) - p.div_euclid(100)
        }
    };

    let end_position = match rotation.dir {
        Direction::L => (p - d).rem_euclid(100),
        Direction::R => (p + d).rem_euclid(100),
    };

    (end_position, zero_passes)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
