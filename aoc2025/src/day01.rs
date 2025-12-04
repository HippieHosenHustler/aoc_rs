use shared::{read_lines, DaySolution};

pub struct Day01;

impl DaySolution for Day01 {
    fn solve(input_file: &str) {
        let mut position = STARTING_POSITION;
        let mut zero_counter = 0;
        let mut zero_passes = 0;

        let lines = read_lines(input_file);

        for line in lines {
            let rotation = parse_rotation(&line);
            let result = get_next_position(position, rotation);
            position = result.0;
            zero_passes += result.1;
            if position == 0 {
                zero_counter += 1;
            }
        }

        println!("Number of zeroes (part 1 solution): {}", zero_counter);
        println!("Zero passes (part 2 solution): {}", zero_passes);
    }
}

struct Rotation {
    dir: Direction,
    value: i32,
}

enum Direction {
    L,
    R,
}

const TRACK_SIZE: i32 = 100;
const STARTING_POSITION: i32 = 50;

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

type PositionResult = (i32, i32);
/**
 * Calculates the next posiotion and counts how many times we cross position 0.
 * on a circular Track of length TRACK_SIZE.
 *
 * @returns (end_position, zero_passes)
 */
fn get_next_position(current_position: i32, rotation: Rotation) -> PositionResult {
    let distance = rotation.value;

    let zero_passes = match rotation.dir {
        Direction::L => {
            // Moving left: count multiples of 100 in range [p-d, p)
            // Formula: floor((p-1)/100) - floor((p-d-1)/100)
            (current_position - 1).div_euclid(TRACK_SIZE)
                - (current_position - distance - 1).div_euclid(TRACK_SIZE)
        }
        Direction::R => {
            // Moving right: count multiples of 100 in range (p, p+d]
            // Formula: floor((p+d)/100) - floor(p/100)
            (current_position + distance).div_euclid(100) - current_position.div_euclid(100)
        }
    };

    let end_position = match rotation.dir {
        Direction::L => (current_position - distance).rem_euclid(TRACK_SIZE),
        Direction::R => (current_position + distance).rem_euclid(TRACK_SIZE),
    };

    (end_position, zero_passes)
}
